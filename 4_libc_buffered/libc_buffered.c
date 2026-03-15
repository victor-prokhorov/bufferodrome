#include <stdio.h>

int main() {
	for (int i = 0; i < 10; i++) {
		printf("line %d buffered\n", i);
	}
	fprintf(stderr, "stderr\n");
	fprintf(stderr, "is unbuffered\n");
	fflush(stdout);
	return 0;
}
