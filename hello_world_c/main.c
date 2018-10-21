#include <monotron.h>

int monotron_main(void) {
	puts("Hello from RAM!\n");
	puts("This is a \x1BRRed\x1BW text on a new line.\n");
	puts("Press a key to exit...\n");
	getchar();
	return 1;
}
