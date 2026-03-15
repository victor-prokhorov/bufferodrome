CC      = gcc
CFLAGS  = -O0 -g

c_setvbuf: c_setvbuf.c
	$(CC) $(CFLAGS) -o c_setvbuf c_setvbuf.c

clean:
	rm -f c_setvbuf

strace: c_setvbuf
	strace -e write ./c_setvbuf
