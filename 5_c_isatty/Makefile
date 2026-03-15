CC      = gcc
CFLAGS  = -O0 -g

c_isatty: c_isatty.c
	$(CC) $(CFLAGS) -o c_isatty c_isatty.c

clean:
	rm -f c_isatty

strace-tty: c_isatty
	strace -e trace=write,ioctl ./c_isatty

strace-pipe: c_isatty
	strace -f -e trace=write,ioctl bash -c './c_isatty | cat'
