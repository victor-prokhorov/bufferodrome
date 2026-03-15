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
