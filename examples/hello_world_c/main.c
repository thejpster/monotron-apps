#include <monotron.h>

#ifdef LINUX_BUILD
int main(int argc, const char** argv) {
	init();
	int result = monotron_main();
	deinit();
	return result;
}
#endif

int monotron_main(void) {
	puts("Hello from RAM!\n");
	puts("This is a \x1BRRed\x1BW text on a new line.\n");
	puts("Press a key to exit...\n");
	getchar();
	return 1;
}
