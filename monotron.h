#include <stdint.h>

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

int main(void);
int putchar(char ch);
int puts(const char* s);
int getchar(void);
void wfvbi(void);
int rand(void);
void srand(unsigned int seed);
int kbhit(void);
void move_cursor(unsigned char row, unsigned char col);
void itoa(int n, char s[]);
int play(uint32_t frequency, channel_t channel, waveform_t waveform, uint8_t volume);

#define NULL ((void*) 0)
#define RAND_MAX 32768
#define FRAMES_PER_SECOND 60
