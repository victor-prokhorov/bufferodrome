// Proof of BufferedWriter<W> allocation structure with the stdio_buffering API.
//
// BufferedWriter<StdoutRaw> = BufWriter<StdoutRaw> (one Vec<u8>) + BufferingMode (enum, zero alloc)
// set_buffering_mode()  → flips enum only, no realloc
// BufWriter::new(lock)  → adds a second independent Vec<u8> on top
//
// Build (requires stage1 rustc with custom stdlib):
//   STAGE1=$(rustup run stage1 rustc --print sysroot)
//   RUSTFLAGS="--sysroot $STAGE1" \
//   rustup run stage1 cargo build --bin stdio_buffering_proof --features stage1
//
//   NOTE: do NOT use -C prefer-dynamic here. With dynamic libstd.so the shared library has its
//   own __rust_alloc wiring and our Counter GlobalAlloc never sees stdlib Vec allocations.
//   Static linking routes every alloc through Counter so the deltas are accurate.
//
// Run:
//   strace -e write ./target/debug/stdio_buffering_proof 2>&1

#![cfg_attr(feature = "stage1", feature(stdio_buffering))]

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};

static ALLOCS: AtomicUsize = AtomicUsize::new(0);
static BYTES: AtomicUsize = AtomicUsize::new(0);
static FREES: AtomicUsize = AtomicUsize::new(0);

struct Counter;
unsafe impl GlobalAlloc for Counter {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, SeqCst);
        BYTES.fetch_add(l.size(), SeqCst);
        unsafe { System.alloc(l) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, l: Layout) {
        FREES.fetch_add(1, SeqCst);
        unsafe { System.dealloc(ptr, l) }
    }
}
#[global_allocator]
static GA: Counter = Counter;

#[derive(Clone, Copy)]
struct Snap {
    allocs: usize,
    bytes: usize,
    frees: usize,
}
impl Snap {
    fn now() -> Self {
        Self { allocs: ALLOCS.load(SeqCst), bytes: BYTES.load(SeqCst), frees: FREES.load(SeqCst) }
    }
    fn delta(self, label: &str) -> Self {
        let n = Self::now();
        let live = n.allocs - n.frees;
        eprintln!(
            "  [{label}]\n    +{} alloc(s)  +{} bytes  +{} free(s)  live={}",
            n.allocs - self.allocs,
            n.bytes - self.bytes,
            n.frees - self.frees,
            live,
        );
        n
    }
}

#[cfg(not(feature = "stage1"))]
fn main() {
    eprintln!("build with --features stage1 using stage1 rustc — see file header");
}

#[cfg(feature = "stage1")]
fn main() {
    use std::io::{BufWriter, BufferedWrite, BufferingMode, Write};

    eprintln!("warm up stderr");
    let s = Snap::now();

    // ── phase 1: default stdout — BufferedWriter<StdoutRaw> init ────────────
    eprintln!("\n=== phase 1: first stdout().lock()");
    eprintln!("    BufferedWriter::new() calls BufWriter::new() → ONE Vec<u8> allocated");
    eprintln!("    mode: BufferingMode (enum) is stored inline — zero extra alloc");
    {
        let lock = std::io::stdout().lock();
        let s = s.delta("after stdout().lock()  → BufferedWriter init: +1 Vec<u8>");
        drop(lock);
        s.delta("after drop");
    }
    let s = Snap::now();

    // ── phase 2: set_buffering_mode — enum flip, zero allocation ────────────
    eprintln!("\n=== phase 2: set_buffering_mode — only flips BufferingMode enum");
    eprintln!("    no Vec realloc, no new allocation regardless of mode");
    {
        let mut lock = std::io::stdout().lock();
        let s = s.delta("after stdout().lock() (OnceLock already init)");

        lock.set_buffering_mode(BufferingMode::Buffered);
        let s = s.delta("after set_buffering_mode(Buffered)   → +0 allocs");

        // block-buffered: writeln! does NOT flush — no write syscall until flush/drop
        use std::io::Write;
        writeln!(lock, "buffered-line1").unwrap();
        writeln!(lock, "buffered-line2").unwrap();
        let s = s.delta("after 2 writeln! (Buffered: data in Vec, 0 write() syscalls yet)");

        lock.set_buffering_mode(BufferingMode::LineBuffered);
        s.delta("after set_buffering_mode(LineBuffered) → +0 allocs, flush triggered on next newline");
        // LineBuffered: writeln! flushes immediately
        writeln!(lock, "linebuffered-line1").unwrap();
        writeln!(lock, "linebuffered-line2").unwrap();
        Snap::now().delta("after 2 writeln! (LineBuffered: 2 write() syscalls)");

        lock.set_buffering_mode(BufferingMode::Unbuffered);
        Snap::now().delta("after set_buffering_mode(Unbuffered) → +0 allocs");
        // Unbuffered: bypasses inner Vec entirely, each write goes straight to fd1
        writeln!(lock, "unbuffered-line1").unwrap();
        writeln!(lock, "unbuffered-line2").unwrap();
        Snap::now().delta("after 2 writeln! (Unbuffered: 2*2=4 write() syscalls — content + \\n separate)");
    }
    let s = Snap::now();

    // ── phase 3: set_buffer_capacity — resizes the ONE shared Vec in place ──
    eprintln!("\n=== phase 3: set_buffer_capacity — same Vec, just resized");
    eprintln!("    initial capacity: 8192 (from OnceLock init)");
    eprintln!("    shrink / grow both operate on the SAME allocation — live count never changes");
    {
        let mut lock = std::io::stdout().lock();
        lock.set_buffering_mode(BufferingMode::LineBuffered); // back to sane default
        let s = s.delta(&format!(
            "after stdout().lock()  capacity={}",
            lock.buffer_capacity()
        ));

        // shrink: Vec::shrink_to → dealloc old + alloc smaller → +1 alloc +1 free, live same
        lock.set_buffer_capacity(256);
        let s = s.delta(&format!(
            "after set_buffer_capacity(256)  capacity={}  → realloc: +1 alloc +1 free, live unchanged",
            lock.buffer_capacity()
        ));

        // grow beyond original: Vec::reserve → dealloc old + alloc larger → +1 alloc +1 free, live same
        lock.set_buffer_capacity(32768);
        let s = s.delta(&format!(
            "after set_buffer_capacity(32768)  capacity={}  → realloc: +1 alloc +1 free, live unchanged",
            lock.buffer_capacity()
        ));

        // shrink back to default
        lock.set_buffer_capacity(8192);
        s.delta(&format!(
            "after set_buffer_capacity(8192)  capacity={}  → back to default size",
            lock.buffer_capacity()
        ));
    }
    let s = Snap::now();

    // ── phase 4: BufWriter::new(stdout().lock()) on top of BufferedWriter ───
    eprintln!("\n=== phase 4: BufWriter::new(stdout().lock())");
    eprintln!("    outer BufWriter adds a SECOND Vec<u8> on top of the inner one");
    eprintln!("    inner BufferedWriter still exists — now two live Vec<u8>s");
    {
        // restore inner to LineBuffered so we can see it being overridden
        {
            let mut lock = std::io::stdout().lock();
            lock.set_buffering_mode(BufferingMode::LineBuffered);
        }

        let mut w = BufWriter::new(std::io::stdout().lock());
        let s = s.delta("after BufWriter::new()  → outer Vec<u8> allocated: +1 alloc +8192 bytes");

        writeln!(w, "outer-line1").unwrap();
        writeln!(w, "outer-line2").unwrap();
        writeln!(w, "outer-line3").unwrap();
        let s = s.delta("after 3 writeln! — outer Vec buffers all, 0 write() syscalls\n    (inner LineBuffered never sees individual lines — outer dominates)");

        w.flush().unwrap();
        s.delta("after flush() — 1 write() syscall: outer Vec → inner BufferedWriter → fd1");
    }
    let s = Snap::now();
    s.delta("after BufWriter dropped  → outer Vec freed, inner Vec still alive (back to live=3)");
}
