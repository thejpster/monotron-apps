#include <monotron.h>
#include "cpu.h"

uint8_t g_curkey;

#ifdef LINUX_BUILD
int main(int argc, const char** argv) {
    init();
    int result = monotron_main();
    deinit();
    return result;
}
#endif

int monotron_main(void) {
	reset6502();
#ifdef LINUX_BUILD
	uint16_t x = 0;
#endif
	while (1) {
		exec6502(100);
#ifdef LINUX_BUILD
		// Pump the event queue periodically
		x = x + 1;
		if ( x == 0 )
		{
			wfvbi();
		}
#endif
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
