// Proof that BufWriter::new(stdout().lock()) creates TWO independent heap buffers:
//   - inner: Vec<u8> inside the global LineWriter<StdoutRaw> (allocated on first stdout() call)
//   - outer: Vec<u8> inside your BufWriter (allocated on BufWriter::new)
//
// Verification:
//   cargo build --bin double_buffer_proof
//   ./target/debug/double_buffer_proof        # allocation deltas
//   strace -e write ./target/debug/double_buffer_proof 2>&1  # write syscall pattern

use std::alloc::{GlobalAlloc, Layout, System};
use std::io::{BufWriter, Write};
use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};

// ── counting allocator ──────────────────────────────────────────────────────

static ALLOCS: AtomicUsize = AtomicUsize::new(0);
static ALLOC_BYTES: AtomicUsize = AtomicUsize::new(0);
static FREES: AtomicUsize = AtomicUsize::new(0);

struct Counter;

unsafe impl GlobalAlloc for Counter {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, SeqCst);
        ALLOC_BYTES.fetch_add(l.size(), SeqCst);
        unsafe { System.alloc(l) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, l: Layout) {
        FREES.fetch_add(1, SeqCst);
        unsafe { System.dealloc(ptr, l) }
    }
}

#[global_allocator]
static GA: Counter = Counter;

// ── snapshot helper ─────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
struct Snap {
    allocs: usize,
    bytes: usize,
    frees: usize,
}

impl Snap {
    fn now() -> Self {
        Self {
            allocs: ALLOCS.load(SeqCst),
            bytes: ALLOC_BYTES.load(SeqCst),
            frees: FREES.load(SeqCst),
        }
    }

    // prints delta since self, returns new snapshot
    fn delta(self, label: &str) -> Self {
        let n = Self::now();
        let da = n.allocs - self.allocs;
        let db = n.bytes - self.bytes;
        let df = n.frees - self.frees;
        let live = n.allocs - n.frees;
        eprintln!("  [{label}]");
        eprintln!("    +{da} alloc(s)  +{db} bytes  +{df} free(s)  live={live}");
        n
    }
}

// ── main ────────────────────────────────────────────────────────────────────

fn main() {
    // warm up stderr so its own init doesn't pollute our counts
    eprintln!("=== double buffer proof ===\n");

    let s = Snap::now();

    // ── phase 1: inner buffer only ──────────────────────────────────────────
    eprintln!("phase 1: stdout().lock() directly  (inner buffer only)");
    eprintln!("  expected: +1 alloc for the inner Vec<u8> (8192 bytes) on first stdout() call");
    {
        let mut lock = std::io::stdout().lock();
        let s = s.delta("after stdout().lock()");

        // line-buffered: each writeln! flushes immediately → one syscall each
        writeln!(lock, "phase1-line1").unwrap();
        writeln!(lock, "phase1-line2").unwrap();
        writeln!(lock, "phase1-line3").unwrap();
        s.delta("after 3 writeln! (line-buffered: 3 separate write syscalls in strace)");
    }
    let s = Snap::now();

    // ── phase 2: outer BufWriter wrapping ───────────────────────────────────
    eprintln!("\nphase 2: BufWriter::new(stdout().lock())  (outer + inner buffers)");
    eprintln!("  expected: +1 alloc for the outer Vec<u8> (8192 bytes)");
    eprintln!("  inner Vec from phase 1 still exists — that's TWO live Vecs");
    {
        let mut w = BufWriter::new(std::io::stdout().lock());
        let s = s.delta("after BufWriter::new()");

        writeln!(w, "phase2-line1").unwrap();
        writeln!(w, "phase2-line2").unwrap();
        writeln!(w, "phase2-line3").unwrap();
        s.delta("after 3 writeln! (data in outer Vec, NOT yet in inner, 0 write syscalls)");

        w.flush().unwrap();
        s.delta("after flush() (outer Vec → inner → fd1, 1 write syscall in strace)");
    }
    let s = Snap::now();
    s.delta("after BufWriter dropped (outer Vec freed, inner Vec still alive)");

    // ── phase 3: inner buffer only again ────────────────────────────────────
    eprintln!("\nphase 3: stdout().lock() again  (OnceLock already init, no new alloc)");
    eprintln!("  expected: +0 allocs — inner Vec reused from phase 1");
    {
        let mut lock = std::io::stdout().lock();
        let s = Snap::now();
        s.delta("after stdout().lock() (second time)");

        writeln!(lock, "phase3-line1").unwrap();
        writeln!(lock, "phase3-line2").unwrap();
        Snap::now().delta("after 2 writeln! (line-buffered again, 2 write syscalls)");
    }
}
