#include <stdint.h>
#include <stdbool.h>

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
#define RAND_MAX 32768
#define FRAMES_PER_SECOND 60
#define ELEMOF(x) (sizeof (x) / sizeof (x)[0])
#define NUMELTS ELEMOF

#define TERM_ESCAPE_STR "\x1B"
#define TERM_FG_RED TERM_ESCAPE_STR "R"
#define TERM_FG_GREEN TERM_ESCAPE_STR "G"
#define TERM_FG_BLUE TERM_ESCAPE_STR "B"
#define TERM_FG_BLACK TERM_ESCAPE_STR "K"
#define TERM_FG_WHITE TERM_ESCAPE_STR "W"
#define TERM_FG_YELLOW TERM_ESCAPE_STR "Y"
#define TERM_FG_CYAN TERM_ESCAPE_STR "C"
#define TERM_FG_MAGENTA TERM_ESCAPE_STR "M"
#define TERM_BG_RED TERM_ESCAPE_STR "r"
#define TERM_BG_GREEN TERM_ESCAPE_STR "g"
#define TERM_BG_BLUE TERM_ESCAPE_STR "b"
#define TERM_BG_BLACK TERM_ESCAPE_STR "k"
#define TERM_BG_WHITE TERM_ESCAPE_STR "w"
#define TERM_BG_YELLOW TERM_ESCAPE_STR "y"
#define TERM_BG_CYAN TERM_ESCAPE_STR "c"
#define TERM_BG_MAGENTA TERM_ESCAPE_STR "m"
#define TERM_DOUBLE_UPPER TERM_ESCAPE_STR "^"
#define TERM_DOUBLE_LOWER TERM_ESCAPE_STR "v"
#define TERM_DOUBLE_CANCEL TERM_ESCAPE_STR "-"
#define TERM_CLS TERM_ESCAPE_STR "Z"

// Note frequencies are in Centi-hertz
#define Note_Rest  0
#define Note_C0  1635
#define Note_CsDb0  1732
#define Note_D0  1835
#define Note_DsEb0  1945
#define Note_E0  2060
#define Note_F0  2183
#define Note_FsGb0  2312
#define Note_G0  2450
#define Note_GsAb0  2596
#define Note_A0  2750
#define Note_AsBb0  2914
#define Note_B0  3087
#define Note_C1  3270
#define Note_CsDb1  3465
#define Note_D1  3671
#define Note_DsEb1  3889
#define Note_E1  4120
#define Note_F1  4365
#define Note_FsGb1  4625
#define Note_G1  4900
#define Note_GsAb1  5191
#define Note_A1  5500
#define Note_AsBb1  5827
#define Note_B1  6174
#define Note_C2  6541
#define Note_CsDb2  6930
#define Note_D2  7342
#define Note_DsEb2  7778
#define Note_E2  8241
#define Note_F2  8731
#define Note_FsGb2  9250
#define Note_G2  9800
#define Note_GsAb2  10383
#define Note_A2  11000
#define Note_AsBb2  11654
#define Note_B2  12347
#define Note_C3  13081
#define Note_CsDb3  13859
#define Note_D3  14683
#define Note_DsEb3  15556
#define Note_E3  16481
#define Note_F3  17461
#define Note_FsGb3  18500
#define Note_G3  19600
#define Note_GsAb3  20765
#define Note_A3  22000
#define Note_AsBb3  23308
#define Note_B3  24694
#define Note_C4  26163
#define Note_CsDb4  27718
#define Note_D4  29366
#define Note_DsEb4  31113
#define Note_E4  32963
#define Note_F4  34923
#define Note_FsGb4  36999
#define Note_G4  39200
#define Note_GsAb4  41530
#define Note_A4  44000
#define Note_AsBb4  46616
#define Note_B4  49388
#define Note_C5  52325
#define Note_CsDb5  55437
#define Note_D5  58733
#define Note_DsEb5  62225
#define Note_E5  65925
#define Note_F5  69846
#define Note_FsGb5  73999
#define Note_G5  78399
#define Note_GsAb5  83061
#define Note_A5  88000
#define Note_AsBb5  93233
#define Note_B5  98777
#define Note_C6  104650
#define Note_CsDb6  110873
#define Note_D6  117466
#define Note_DsEb6  124451
#define Note_E6  131851
#define Note_F6  139691
#define Note_FsGb6  147998
#define Note_G6  156798
#define Note_GsAb6  166122
#define Note_A6  176000
#define Note_AsBb6  186466
#define Note_B6  197553
#define Note_C7  209300
#define Note_CsDb7  221746
#define Note_D7  234932
#define Note_DsEb7  248902
#define Note_E7  263702
#define Note_F7  279383
#define Note_FsGb7  295996
#define Note_G7  313596
#define Note_GsAb7  332244
#define Note_A7  352000
#define Note_AsBb7  372931
#define Note_B7  395107
#define Note_C8  418601
#define Note_CsDb8  443492
#define Note_D8  469863
#define Note_DsEb8  497803
#define Note_E8  527404
#define Note_F8  558765
#define Note_FsGb8  591991
#define Note_G8  627193
#define Note_GsAb8  664488
#define Note_A8  704000
#define Note_AsBb8  745862
#define Note_B8  790213

/* The newlib version seems to crash...*/
char * monotron_utoa(unsigned int value, char* str, int base);

/* Entry point to the user's program */
int monotron_main(void);
/* Write a single character to the screen at the current cursor position. */
int putchar(int ch);
/* Write a connected sixel to the screen. Assumes you have the Teletext font selected. */
void put_connected_sixel(uint8_t ch);
/* Write a separated sixel to the screen. Assumes you have the Teletext font selected. */
void put_separated_sixel(uint8_t ch);
/* Write a null-terminated string to the screen at the current cursor position. */
int puts(const char* s);
/* Show/hide cursor */
void set_cursor_visible(bool enabled);
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
/* Configure one channel of the synthesiser to continuously play a note */
int play(uint32_t frequency, channel_t channel, waveform_t waveform, uint8_t volume);
/* Switch to the CodePage 850 font */
void font_normal(void);
/* Switch to the Teletext font */
void font_teletext(void);
/* Supply 4096 bytes of font data (16 bytes per char, 256 chars) */
void font_custom(const void* p_font);
/* Fetch joystick state */
uint8_t get_joystick(void);
/* Check joystick state */
bool joystick_is_up(uint8_t state);
bool joystick_is_down(uint8_t state);
bool joystick_is_left(uint8_t state);
bool joystick_is_right(uint8_t state);
bool joystick_fire_pressed(uint8_t state);
