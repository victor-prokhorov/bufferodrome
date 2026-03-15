CC      = gcc
CFLAGS  = -O0 -g

write: write.c
	$(CC) $(CFLAGS) -o write write.c

clean:
	rm -f write

strace: write
	strace -e write ./write
