```bash
$ make strace
gcc -O0 -g -o write write.c
strace -e write ./write
write(1, "stdout msg content", 18stdout msg content)      = 18
write(2, "stderr msg content", 18stderr msg content)      = 18
+++ exited with 0 +++
```
