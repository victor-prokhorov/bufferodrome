#!/usr/bin/env bash
set -euo pipefail

STAGE1=$(rustup run stage1 rustc --print sysroot)

step() { echo; echo "=== $*"; }
ok()   { echo "  OK  $*"; }

# ── double_buffer_proof (stable rustc, no feature needed) ───────────────────
step "build double_buffer_proof (stable rustc)"
cargo build --bin double_buffer_proof
ok "built"

step "run double_buffer_proof (allocation deltas)"
./target/debug/double_buffer_proof

# ── stdio_buffering_proof (stage1 rustc + --features stage1) ────────────────
step "build stdio_buffering_proof (stage1 rustc, static std so GlobalAlloc intercepts everything)"
# No -C prefer-dynamic here: with dynamic linking libstd.so has its own __rust_alloc and our
# Counter allocator never sees stdlib Vec allocations. Static linking routes all allocs through us.
RUSTFLAGS="--sysroot $STAGE1" \
    rustup run stage1 cargo build --bin stdio_buffering_proof --features stage1
ok "built"

step "run stdio_buffering_proof (allocation deltas)"
./target/debug/stdio_buffering_proof
