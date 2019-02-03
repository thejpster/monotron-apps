/**
 * Monotron slide-demo / demonstration.
 *
 * Copyright (c) Jonathan 'theJPster' Pallant 2019
 *
 * Available under the MIT or Apache 2.0 licence, at your option.
 */

/******************************************************************************
 *
 * System Header Files
 *
 *****************************************************************************/
#include <stdbool.h>
#include <string.h>

/******************************************************************************
 *
 * Interface Header Files
 *
 *****************************************************************************/
#include <monotron.h>

/******************************************************************************
 *
 * Local Header Files
 *
 *****************************************************************************/
// None

/******************************************************************************
 *
 * Macros
 *
 *****************************************************************************/
#define PAGE_SECONDS 10U

/******************************************************************************
 *
 * Public Data
 *
 *****************************************************************************/
// None

/******************************************************************************
 *
 * Private Types
 *
 *****************************************************************************/
// None

/******************************************************************************
 *
 * Private Function Prototypes
 *
 *****************************************************************************/
static void delay_frames(uint32_t frames);

/******************************************************************************
 *
 * Private Data
 *
 *****************************************************************************/
static const char PAGE_1[] = ""\
	TERM_CLS TERM_FG_YELLOW TERM_DOUBLE_UPPER \
	"Welcome to the Monotron!\n" \
	TERM_FG_RED TERM_DOUBLE_LOWER \
	"Welcome to the Monotron!\n" \
	TERM_DOUBLE_CANCEL TERM_FG_WHITE "\n" \
	"I am a Cortex-M4 powered home computer. My ROM\n" \
	"is written in the Rust Programming Language\n" \
	"(https://www.rust-lang.org).\n" \
	"\n" \
	"I exist to show you can you take a modern\n" \
	"language and use it to write hard-real time\n" \
	"embedded systems. Plus retro computing is fun!\n" \
	"\n" \
	"I drive an 800x600 VGA display through 3 SPI\n" \
	"peripherals at 20 MHz.\n" \
	"\n" \
	"  " TERM_FG_YELLOW "o" TERM_FG_CYAN " 400x600 effective resolution\n" \
	"  " TERM_FG_YELLOW "o" TERM_FG_CYAN " 8 colours " TERM_FG_RED "R" TERM_FG_GREEN "G" TERM_FG_BLUE "B" TERM_FG_CYAN "C" TERM_FG_MAGENTA "M" TERM_FG_YELLOW "Y" TERM_FG_WHITE "\n" \
	"  " TERM_FG_YELLOW "o" TERM_FG_CYAN " 48 cols by 36 rows in text mode\n" \
	"  " TERM_FG_YELLOW "o" TERM_FG_CYAN " 8px by 16px characters\n" \
	"  " TERM_FG_YELLOW "o" TERM_FG_CYAN " CodePage 850 character set \x03\x04\x05\x06\n" \
	"  " TERM_FG_YELLOW "o" TERM_FG_CYAN " Alternative 'Teletext' character set\n" \
	"  " TERM_FG_YELLOW "o" TERM_FG_CYAN " Support for custom fonts in RAM\n" \
	TERM_FG_WHITE "\n";

static const char PAGE_2[] = "" \
	TERM_CLS TERM_FG_YELLOW TERM_DOUBLE_UPPER "Features:\n" \
	TERM_FG_GREEN TERM_DOUBLE_LOWER "Features:\n" \
	TERM_DOUBLE_CANCEL TERM_FG_WHITE "\n" \
	"  " TERM_FG_YELLOW "o" TERM_FG_CYAN " 80 MHz Cortex-M4F\n" \
	"  " TERM_FG_YELLOW "o" TERM_FG_CYAN " TI TM4C123 Microcontroller\n" \
	"  " TERM_FG_YELLOW "o" TERM_FG_CYAN " 32 KiB SRAM (8 KiB reserved for OS)\n" \
	"  " TERM_FG_YELLOW "o" TERM_FG_CYAN " 256 KiB Flash ROM\n" \
	"  " TERM_FG_YELLOW "o" TERM_FG_CYAN " Simple C and Rust APIs for apps\n" \
	"  " TERM_FG_YELLOW "o" TERM_FG_CYAN " Serial Input @ 115200 bps\n" \
	"  " TERM_FG_YELLOW "o" TERM_FG_CYAN " 9-pin Atari Joystick interface\n" \
	"  " TERM_FG_YELLOW "o" TERM_FG_CYAN " 8-bit Mono Audio Output\n" \
	"  " TERM_FG_YELLOW "o" TERM_FG_CYAN " 3-channel 4-waveform Synth\n" \
	"  " TERM_FG_YELLOW "o" TERM_FG_CYAN " SD Card Interface and FAT16/32 support\n" \
	"  " TERM_FG_YELLOW "o" TERM_FG_CYAN " PS/2 Keyboard Input*\n" \
	"  " TERM_FG_YELLOW "o" TERM_FG_CYAN " PS/2 Mouse Input*\n" \
	"  " TERM_FG_YELLOW "o" TERM_FG_CYAN " Centronics Parallel Port*\n" \
	"  " TERM_FG_YELLOW "o" TERM_FG_CYAN " Battery Backed Real-time Clock*\n" \
	"  " TERM_FG_YELLOW "o" TERM_FG_CYAN " MIDI In/Out/Thru*\n" \
	TERM_FG_WHITE "\n" \
	"* available on the Monotron PCB, coming soon!\n" \
	"\n" \
	"Software ports so far include TinyBASIC, Snake\n" \
	"and a 6502 Emulator running Enhanced BASIC.\n" \
	"\n";

static const char PAGE_3[] = "" \
	TERM_CLS TERM_FG_GREEN TERM_DOUBLE_UPPER "Learn more:\n" \
	TERM_FG_RED TERM_DOUBLE_LOWER "Learn more:\n" \
	TERM_DOUBLE_CANCEL "\n" \
	"  " TERM_FG_YELLOW "o" TERM_FG_CYAN " https://github.com/thejpster/monotron\n" \
	"\n" \
	"  " TERM_FG_YELLOW "o" TERM_FG_CYAN " https://github.com/thejpster/monotron-apps\n" \
	"\n" \
	"  " TERM_FG_YELLOW "o" TERM_FG_CYAN " https://github.com/rust-embedded\n" \
	"\n" \
	"  " TERM_FG_YELLOW "o" TERM_FG_CYAN " https://www.rust-lang.org\n" \
	"\n" \
	"  " TERM_FG_YELLOW "o" TERM_FG_CYAN " https://twitter.com/therealjpster\n" \
	TERM_FG_WHITE "\n";

static bool g_run = true;

/******************************************************************************
 *
 * Public Functions
 *
 *****************************************************************************/

/**
 * Entry point for Monotron applications.
 *
 * @return 0 on successful termination, any else means there was an error.
 */
int monotron_main(void) {
	const char* pages[] = { PAGE_1, PAGE_2, PAGE_3 };
	int result = 0;
	while(g_run) {
		for(size_t idx = 0; idx < ELEMOF(pages); idx++) {
			puts(pages[idx]);
			delay_frames(FRAMES_PER_SECOND * PAGE_SECONDS);
			if (!g_run) {
				break;
			}
		}
	}
	return result;
}

/******************************************************************************
 *
 * Private Functions
 *
 *****************************************************************************/

/**
 * Delay for a certain number of 60 Hz video frames. Uses the `wfvbi` syscall.
 *
 * @param frames Number of 60 Hz video frames to pause for
 */
static void delay_frames(uint32_t frames) {
	for(uint32_t x = 0; x < frames; x++) {
		if (kbhit()) {
			int ch = getchar();
			// Space bar only skips a slide
			// Anything else to quit
			if (ch != ' ') {
				g_run = false;
			}
			return;
		}
		wfvbi();
	}
}

/******************************************************************************
 *
 * End of File
 *
 *****************************************************************************/
