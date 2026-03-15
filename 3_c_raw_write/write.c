#include <unistd.h>

int main(void) {
	const char stdout_msg[] = "stdout msg content";
	const char stderr_msg[] = "stderr msg content";
	write(1, stdout_msg, sizeof(stdout_msg) - 1);
	write(2, stderr_msg, sizeof(stderr_msg) - 1);
	return 0;
}
