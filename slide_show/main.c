#include <monotron.h>
#include <stdbool.h>

#define PAGE_SECONDS 10

static const char PAGE_1[] = "" \
"\eB\eyWelcome to the Monotron!\eW\ek\n" \
"\n" \
"I am a Cortex-M4 powered home computer.\n" \
"My ROM is written in the Rust Programming\n" \
"Language (www.rust-lang.org).\n" \
"\n" \
"I exist to show you can you take a modern\n" \
"language and use it to write hard-real time\n" \
"embedded systems.\n" \
"\n" \
"I drive an 800x600 VGA display through 3 SPI.\n" \
"peripherals at 20 MHz.\n" \
"\n" \
"o 400x600 effective resolution\n" \
"o 8 colours \eRR\eGG\eBB\eCC\eMM\eYY\eW\n" \
"o 48 x 36 text mode\n" \
"o 8 x 16 characters (from FreeBSD)\n" \
"o CodePage 850 character set \x03\x04\x05\x06\n" \
"\n" \
"\n" \
"\n" \
"\n" \
"\n" \
"\n" \
"\n" \
"\n" \
"\n" \
"\n" \
"\n" \
"\n" \
"\n" \
"\n" \
"\n" \
"\n";

static const char PAGE_2[] = "" \
"Features:\n" \
"\n" \
"o 80 MHz Cortex-M4F\n" \
"o TI TM4C123 Microcontroller\n" \
"o 32 KiB SRAM\n" \
"o 256 KiB Flash ROM\n" \
"o Serial Input @ 115200 bps\n" \
"o PS/2 Keyboard Input*\n" \
"o 8-bit mono audio output*\n" \
"\n" \
"* coming soon\n" \
"\n" \
"\n" \
"\n" \
"\n" \
"\n" \
"\n" \
"\n" \
"\n" \
"\n" \
"\n" \
"\n" \
"\n" \
"\n" \
"\n" \
"\n" \
"\n" \
"\n" \
"\n" \
"\n" \
"\n" \
"\n" \
"\n" \
"\n" \
"\n";

static void delay_frames(unsigned int frames);
static bool run = true;

int main(void) {
	while(true) {
		puts(PAGE_1);
		delay_frames(FRAMES_PER_SECOND * PAGE_SECONDS);
		if (!run) {
			return 0;
		}
		puts(PAGE_2);
		delay_frames(FRAMES_PER_SECOND * PAGE_SECONDS);
		if (!run) {
			return 0;
		}
	}
	return 1;
}

static void delay_frames(unsigned int frames) {
	for(unsigned int x = 0; x < frames; x++) {
		if (kbhit()) {
			getchar();
			run = false;
			return;
		}
		wfvbi();
	}
}
