```bash
$ strace -e write ./target/debug/println > /dev/null
write(1, "line 0\n", 7)                 = 7
write(1, "line 1\n", 7)                 = 7
write(1, "line 2\n", 7)                 = 7
write(1, "line 3\n", 7)                 = 7
write(1, "line 4\n", 7)                 = 7
+++ exited with 0 +++
```

```bash
$ strace -e write ./target/debug/write_trait
write(1, "line 0\n", 7line 0
)                 = 7
write(1, "line 1\n", 7line 1
)                 = 7
write(1, "line 2\n", 7line 2
)                 = 7
write(1, "line 3\n", 7line 3
)                 = 7
write(1, "line 4\n", 7line 4
)                 = 7
+++ exited with 0 +++
```

```bash
$ strace -e write ./target/debug/stderr
write(2, "printing line ", 14printing line )          = 14
write(2, "0", 10)                        = 1
write(2, " to standard output\n", 20 to standard output
)   = 20
write(2, "printing line ", 14printing line )          = 14
write(2, "1", 11)                        = 1
write(2, " to standard output\n", 20 to standard output
)   = 20
write(2, "printing line ", 14printing line )          = 14
write(2, "2", 12)                        = 1
write(2, " to standard output\n", 20 to standard output
)   = 20
+++ exited with 0 +++
```

```bash
$ strace -e write ./target/debug/bufwriter
write(1, "line 0\nline 1\nline 2\nline 3\nline"..., 35line 0
line 1
line 2
line 3
line 4
) = 35
+++ exited with 0 +++
```

```bash
$ ./run.sh

=== build double_buffer_proof (stable rustc)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
  OK  built

=== run double_buffer_proof (allocation deltas)
=== double buffer proof ===

phase 1: stdout().lock() directly  (inner buffer only)
  expected: +1 alloc for the inner Vec<u8> (8192 bytes) on first stdout() call
  [after stdout().lock()]
    +1 alloc(s)  +1024 bytes  +0 free(s)  live=3
phase1-line1
phase1-line2
phase1-line3
  [after 3 writeln! (line-buffered: 3 separate write syscalls in strace)]
    +0 alloc(s)  +0 bytes  +0 free(s)  live=3

phase 2: BufWriter::new(stdout().lock())  (outer + inner buffers)
  expected: +1 alloc for the outer Vec<u8> (8192 bytes)
  inner Vec from phase 1 still exists — that's TWO live Vecs
  [after BufWriter::new()]
    +1 alloc(s)  +8192 bytes  +0 free(s)  live=4
  [after 3 writeln! (data in outer Vec, NOT yet in inner, 0 write syscalls)]
    +0 alloc(s)  +0 bytes  +0 free(s)  live=4
phase2-line1
phase2-line2
phase2-line3
  [after flush() (outer Vec → inner → fd1, 1 write syscall in strace)]
    +0 alloc(s)  +0 bytes  +0 free(s)  live=4
  [after BufWriter dropped (outer Vec freed, inner Vec still alive)]
    +0 alloc(s)  +0 bytes  +0 free(s)  live=3

phase 3: stdout().lock() again  (OnceLock already init, no new alloc)
  expected: +0 allocs — inner Vec reused from phase 1
  [after stdout().lock() (second time)]
    +0 alloc(s)  +0 bytes  +0 free(s)  live=3
phase3-line1
phase3-line2
  [after 2 writeln! (line-buffered again, 2 write syscalls)]
    +0 alloc(s)  +0 bytes  +0 free(s)  live=3

=== build stdio_buffering_proof (stage1 rustc, static std so GlobalAlloc intercepts everything)
info: `cargo` is unavailable for the active toolchain
info: falling back to "/home/victorprokhorov/.rustup/toolchains/beta-aarch64-unknown-linux-gnu/bin/cargo"
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
  OK  built

=== run stdio_buffering_proof (allocation deltas)
warm up stderr

=== phase 1: first stdout().lock()
    BufferedWriter::new() calls BufWriter::new() → ONE Vec<u8> allocated
    mode: BufferingMode (enum) is stored inline — zero extra alloc
  [after stdout().lock()  → BufferedWriter init: +1 Vec<u8>]
    +1 alloc(s)  +8192 bytes  +0 free(s)  live=3
  [after drop]
    +0 alloc(s)  +0 bytes  +0 free(s)  live=3

=== phase 2: set_buffering_mode — only flips BufferingMode enum
    no Vec realloc, no new allocation regardless of mode
  [after stdout().lock() (OnceLock already init)]
    +0 alloc(s)  +0 bytes  +0 free(s)  live=3
  [after set_buffering_mode(Buffered)   → +0 allocs]
    +0 alloc(s)  +0 bytes  +0 free(s)  live=3
  [after 2 writeln! (Buffered: data in Vec, 0 write() syscalls yet)]
    +0 alloc(s)  +0 bytes  +0 free(s)  live=3
  [after set_buffering_mode(LineBuffered) → +0 allocs, flush triggered on next newline]
    +0 alloc(s)  +0 bytes  +0 free(s)  live=3
buffered-line1
buffered-line2
linebuffered-line1
linebuffered-line2
  [after 2 writeln! (LineBuffered: 2 write() syscalls)]
    +0 alloc(s)  +0 bytes  +0 free(s)  live=3
  [after set_buffering_mode(Unbuffered) → +0 allocs]
    +0 alloc(s)  +0 bytes  +0 free(s)  live=3
unbuffered-line1
unbuffered-line2
  [after 2 writeln! (Unbuffered: 2*2=4 write() syscalls — content + \n separate)]
    +0 alloc(s)  +0 bytes  +0 free(s)  live=3

=== phase 3: set_buffer_capacity — same Vec, just resized
    initial capacity: 8192 (from OnceLock init)
    shrink / grow both operate on the SAME allocation — live count never changes
  [after stdout().lock()  capacity=8192]
    +1 alloc(s)  +64 bytes  +0 free(s)  live=4
  [after set_buffer_capacity(256)  capacity=256  → realloc: +1 alloc +1 free, live unchanged]
    +2 alloc(s)  +432 bytes  +2 free(s)  live=4
  [after set_buffer_capacity(32768)  capacity=32512  → realloc: +1 alloc +1 free, live unchanged]
    +2 alloc(s)  +32692 bytes  +2 free(s)  live=4
  [after set_buffer_capacity(8192)  capacity=8192  → back to default size]
    +2 alloc(s)  +8328 bytes  +2 free(s)  live=4

=== phase 4: BufWriter::new(stdout().lock())
    outer BufWriter adds a SECOND Vec<u8> on top of the inner one
    inner BufferedWriter still exists — now two live Vec<u8>s
  [after BufWriter::new()  → outer Vec<u8> allocated: +1 alloc +8192 bytes]
    +1 alloc(s)  +8192 bytes  +0 free(s)  live=4
  [after 3 writeln! — outer Vec buffers all, 0 write() syscalls
    (inner LineBuffered never sees individual lines — outer dominates)]
    +0 alloc(s)  +0 bytes  +0 free(s)  live=4
outer-line1
outer-line2
outer-line3
  [after flush() — 1 write() syscall: outer Vec → inner BufferedWriter → fd1]
    +0 alloc(s)  +0 bytes  +0 free(s)  live=4
  [after BufWriter dropped  → outer Vec freed, inner Vec still alive (back to live=3)]
    +0 alloc(s)  +0 bytes  +0 free(s)  live=3
$
```
