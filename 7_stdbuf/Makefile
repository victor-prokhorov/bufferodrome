CC      = gcc
CFLAGS  = -O0 -g

all: prog setvbuf_spy.so

prog: prog.c
	$(CC) $(CFLAGS) -o prog prog.c

clean:
	rm -f prog setvbuf_spy.so
