```bash
$ LD_DEBUG=bindings gnustdbuf -oL ./prog 2>&1 | grep setvbuf
     48401:     binding file /usr/libexec/coreutils/libstdbuf.so [0] to /lib/aarch64-linux-gnu/libc.so.6 [0]: normal symbol `setvbuf' [GLIBC_2.17]
$
$ strace -f -e execve,openat gnustdbuf -oL ./prog 2>&1 | grep -E "execve|openat"
execve("/usr/bin/gnustdbuf", ["gnustdbuf", "-oL", "./prog"], 0xfffff0cc4198 /* 65 vars */) = 0
openat(AT_FDCWD, "/etc/ld.so.cache", O_RDONLY|O_CLOEXEC) = 3
openat(AT_FDCWD, "/lib/aarch64-linux-gnu/libc.so.6", O_RDONLY|O_CLOEXEC) = 3
openat(AT_FDCWD, "/usr/lib/locale/locale-archive", O_RDONLY|O_CLOEXEC) = 3
execve("./prog", ["./prog"], 0xb53384d1f4a0 /* 67 vars */) = 0
openat(AT_FDCWD, "/usr/libexec/coreutils/libstdbuf.so", O_RDONLY|O_CLOEXEC) = 3
openat(AT_FDCWD, "/etc/ld.so.cache", O_RDONLY|O_CLOEXEC) = 3
openat(AT_FDCWD, "/lib/aarch64-linux-gnu/libc.so.6", O_RDONLY|O_CLOEXEC) = 3
```
