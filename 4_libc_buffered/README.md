```bash
$ make strace-pipe
gcc -O0 -g -o libc_buffered libc_buffered.c
strace -f -e write bash -c './libc_buffered | cat'
strace: Process 37564 attached
strace: Process 37565 attached
[pid 37564] write(2, "stderr\n", 7stderr
)     = 7
[pid 37564] write(2, "is unbuffered\n", 14is unbuffered
) = 14
[pid 37564] write(1, "line 0 buffered\nline 1 buffered\n"..., 160) = 160
[pid 37564] +++ exited with 0 +++
line 0 buffered
line 1 buffered
line 2 buffered
line 3 buffered
line 4 buffered
line 5 buffered
line 6 buffered
line 7 buffered
line 8 buffered
line 9 buffered
[pid 37565] +++ exited with 0 +++
--- SIGCHLD {si_signo=SIGCHLD, si_code=CLD_EXITED, si_pid=37564, si_uid=1000, si_status=0, si_utime=0, si_stime=0} ---
+++ exited with 0 +++
```

```bash
$ make strace-tty
strace -e write ./libc_buffered
write(1, "line 0 buffered\n", 16line 0 buffered
)       = 16
write(1, "line 1 buffered\n", 16line 1 buffered
)       = 16
write(1, "line 2 buffered\n", 16line 2 buffered
)       = 16
write(1, "line 3 buffered\n", 16line 3 buffered
)       = 16
write(1, "line 4 buffered\n", 16line 4 buffered
)       = 16
write(1, "line 5 buffered\n", 16line 5 buffered
)       = 16
write(1, "line 6 buffered\n", 16line 6 buffered
)       = 16
write(1, "line 7 buffered\n", 16line 7 buffered
)       = 16
write(1, "line 8 buffered\n", 16line 8 buffered
)       = 16
write(1, "line 9 buffered\n", 16line 9 buffered
)       = 16
write(2, "stderr\n", 7stderr
)                 = 7
write(2, "is unbuffered\n", 14is unbuffered
)         = 14
+++ exited with 0 +++
```

```bash
$ make ltrace-non-tty
ltrace -e malloc ./libc_buffered > /dev/null
libc.so.6->malloc(4096)                                                  = 0xc9ac03e0f010
stderr
is unbuffered
+++ exited (status 0) +++
$
$ make ltrace-tty
ltrace -e malloc ./libc_buffered
libc.so.6->malloc(1024)                                                  = 0xaf062c420310
line 0 buffered
line 1 buffered
line 2 buffered
line 3 buffered
line 4 buffered
line 5 buffered
line 6 buffered
line 7 buffered
line 8 buffered
line 9 buffered
stderr
is unbuffered
+++ exited (status 0) +++
```
