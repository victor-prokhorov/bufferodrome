```bash
$ make run
gcc -O0 -g -o file_ptr_internals file_ptr_internals.c
./file_ptr_internals
[after setvbuf] buf: 0 filled, 0 free, 12 total
lazy [after printf("lazy ")] buf: 0 filled, 12 free, 12 total
[after printf("hello ")] buf: 6 filled, 6 free, 12 total
[after printf("world ")] buf: 12 filled, 0 free, 12 total
hello world buffered![after printf("buffered!\n")] buf: 1 filled, 11 free, 12 total

[after fflush] buf: 0 filled, 12 free, 12 total
```
