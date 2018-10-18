#include <stdint.h>
#include <stdbool.h>

typedef unsigned long size_t;

typedef enum channel_t {
	CHANNEL_0 = 0,
	CHANNEL_1 = 1,
	CHANNEL_2 = 2,
} channel_t;

typedef enum waveform_t {
	WAVEFORM_SQUARE = 0,
	WAVEFORM_SINE = 1,
	WAVEFORM_SAWTOOTH = 2,
	WAVEFORM_NOISE = 3,
} waveform_t;

#define MAX_VOLUME 255
#define NULL ((void*) 0)
#define RAND_MAX 32768
#define FRAMES_PER_SECOND 60

/* Entry point to the user's program */
int main(void);
/* Write a single character to the screen at the current cursor position. */
int putchar(char ch);
/* Write a connected sixel to the screen. Assumes you have the Teletext font selected. */
int put_connected_sixel(uint8_t ch);
/* Write a separated sixel to the screen. Assumes you have the Teletext font selected. */
int put_separated_sixel(uint8_t ch);
/* Write a null-terminated string to the screen at the current cursor position. */
int puts(const char* s);
/* Get a key from the keyboard */
int getchar(void);
/* Wait until the screen has been drawn and we're in the vertical blanking interval */
void wfvbi(void);
/* Get a random number from 0 .. RAND_MAX */
int rand(void);
/* See the random number generator */
void srand(unsigned int seed);
/* Return 1 if a key has been pressed (i.e. getchar() won't blocK), 0 otherwise */
int kbhit(void);
/* Set where the next character will appear on screen */
void move_cursor(unsigned char row, unsigned char col);
/* Convert an integer to a string. s must be long enough to hold the integer. */
void itoa(int n, char s[]);
/* Configure one channel of the synthesiser to continuously play a note */
int play(uint32_t frequency, channel_t channel, waveform_t waveform, uint8_t volume);
/* Switch to the CodePage 850 font */
void font_normal(void);
/* Switch to the Teletext font */
void font_teletext(void);
/* Supply 4096 bytes of font data (16 bytes per char, 256 chars) */
void font_custom(const void* p_font);
