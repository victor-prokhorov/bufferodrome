CC      = gcc
CFLAGS  = -O0 -g

file_ptr_internals: file_ptr_internals.c
	$(CC) $(CFLAGS) -o file_ptr_internals file_ptr_internals.c

clean:
	rm -f file_ptr_internals

run: file_ptr_internals
	./file_ptr_internals

strace: file_ptr_internals
	strace -e write ./file_ptr_internals
