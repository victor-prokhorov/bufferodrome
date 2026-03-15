#include <stdio.h>
#include <unistd.h>

int main(void) {
	if (isatty(STDOUT_FILENO)) {
		printf("yes");
	} else {
		printf("non");
	}
	for (int i = 0; i < 5; i++) {
		printf("line %d\n", i);
	}
	return 0;
}
