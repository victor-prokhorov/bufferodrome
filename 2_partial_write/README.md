```bash
$ as write.s -o write.o && ld write.o -o write
$ strace ./write | (sleep 1; cat | wc -c)
execve("./write", ["./write"], 0xfffff572f740 /* 65 vars */) = 0
fcntl(1, F_GETFL)                       = 0x1 (flags O_WRONLY)
fcntl(1, F_SETFL, O_WRONLY|O_NONBLOCK)  = 0
write(1, "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"..., 131072) = 65536
write(2, "partial", 7partial)                  = 7
exit(0)                                 = ?
+++ exited with 0 +++
65536
$
$ sed -i '17,27d'
$
$ as write.s -o write.o && ld write.o -o write
$ strace ./write | (sleep 1; cat | wc -c)
execve("./write", ["./write"], 0xffffdb9ca630 /* 65 vars */) = 0
write(1, "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"..., 131072) = 131072
write(2, "full", 4full)                     = 4
exit(0)                                 = ?
+++ exited with 0 +++
131072
```
