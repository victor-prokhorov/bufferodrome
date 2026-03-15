#include <stdio.h>

static void unbuffered() {
	setvbuf(stdout, NULL, _IONBF, 8192);
	for (int i = 0; i < 5; i++) {
		printf("unbuffered write %d\n", i);
	}
	fflush(stdout);
}

static void line_buffered() {
	setvbuf(stdout, NULL, _IOLBF, 8192);
	for (int i = 0; i < 5; i++) {
		printf("line ");
		printf("buffered %d\n", i);
	}
	fflush(stdout);
}

static void custom_size_buffered() {
	char buf[8];
	setvbuf(stdout, buf, _IOFBF, sizeof(buf));
	for (int i = 0; i < 5; i++) {
		printf("8 byte sized buffered write %d", i);
	}
	fflush(stdout);
}

int main(void) {
	line_buffered();
	custom_size_buffered();
	unbuffered();
	return 0;
}
