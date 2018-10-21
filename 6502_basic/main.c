#include <monotron.h>
#include "cpu.h"

uint8_t g_curkey;

int monotron_main(void) {
	reset6502();
	while (1) {
		exec6502(100);
		if (kbhit()) {
			g_curkey = getchar();
		}
	}
	return 0;
}

void serout(uint8_t val) {
	if (val == 0x08) {
		// BASIC expects a backspace to erase the character
		puts("\x08 \x08");
	} else {
		putchar(val);
	}
}

uint8_t getkey() {
	return g_curkey;
}

void clearkey() {
	g_curkey = 0;
}
