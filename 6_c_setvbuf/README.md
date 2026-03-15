```bash
$ make strace
gcc -O0 -g -o c_setvbuf c_setvbuf.c
strace -e write ./c_setvbuf
write(1, "line buffered 0\n", 16line buffered 0
)       = 16
write(1, "line buffered 1\n", 16line buffered 1
)       = 16
write(1, "line buffered 2\n", 16line buffered 2
)       = 16
write(1, "line buffered 3\n", 16line buffered 3
)       = 16
write(1, "line buffered 4\n", 16line buffered 4
)       = 16
write(1, "8 byte sized buffered write 0", 298 byte sized buffered write 0) = 29
write(1, "8 byte s", 88 byte s)                 = 8
write(1, "ized buf", 8ized buf)                 = 8
write(1, "fered wr", 8fered wr)                 = 8
write(1, "ite 18 b", 8ite 18 b)                 = 8
write(1, "yte size", 8yte size)                 = 8
write(1, "d buffer", 8d buffer)                 = 8
write(1, "ed write", 8ed write)                 = 8
write(1, " 28 byte", 8 28 byte)                 = 8
write(1, " sized b", 8 sized b)                 = 8
write(1, "uffered ", 8uffered )                 = 8
write(1, "write 38", 8write 38)                 = 8
write(1, " byte si", 8 byte si)                 = 8
write(1, "zed buff", 8zed buff)                 = 8
write(1, "ered wri", 8ered wri)                 = 8
write(1, "te 4", 4te 4)                     = 4
write(1, "unbuffered write 0\n", 19unbuffered write 0
)    = 19
write(1, "unbuffered write 1\n", 19unbuffered write 1
)    = 19
write(1, "unbuffered write 2\n", 19unbuffered write 2
)    = 19
write(1, "unbuffered write 3\n", 19unbuffered write 3
)    = 19
write(1, "unbuffered write 4\n", 19unbuffered write 4
)    = 19
+++ exited with 0 +++
```
