```bash
$ as write.s -o write.o
$ ld write.o -o write
```

```bash
$ strace ./write
execve("./write", ["./write"], 0xffffec7824a0 /* 65 vars */) = 0
write(1, "stdout msg content", 18stdout msg content)      = 18
write(2, "stderr msg content", 18stderr msg content)      = 18
exit(0)                                 = ?
+++ exited with 0 +++
```
