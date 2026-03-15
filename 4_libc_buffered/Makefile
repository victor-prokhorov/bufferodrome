CC      = gcc
CFLAGS  = -O0 -g

libc_buffered: libc_buffered.c
	$(CC) $(CFLAGS) -o libc_buffered libc_buffered.c

clean:
	rm -f libc_buffered

strace-pipe: libc_buffered
	strace -f -e write bash -c './libc_buffered | cat'

strace-tty: libc_buffered
	strace -e write ./libc_buffered

ltrace-non-tty: libc_buffered
	ltrace -e malloc ./libc_buffered > /dev/null

ltrace-tty: libc_buffered
	ltrace -e malloc ./libc_buffered
