```bash
$ strace -e write ./target/debug/println > /dev/null
write(1, "line 0\n", 7)                 = 7
write(1, "line 1\n", 7)                 = 7
write(1, "line 2\n", 7)                 = 7
write(1, "line 3\n", 7)                 = 7
write(1, "line 4\n", 7)                 = 7
+++ exited with 0 +++
```
