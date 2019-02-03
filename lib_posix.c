/**
 * Monotron API implementation for Linux/POSIX systems.
 *
 * Assumes you have an ANSI capable terminal.
 *
 * Copyright (c) Jonathan 'theJPster' Pallant 2019
 *
 * Available under the MIT or Apache 2.0 licence, at your option.
 */

#include <monotron.h>
#include <string.h>
#include <stdio.h>
#include <termios.h>
#include <stdlib.h>
#include <time.h>
#include <fcntl.h>
#include <signal.h>

static void setup_console(void);
static void clean_up(void);
static void sigint_handler(int dummy);

static struct termios oldt;

int main(int argc, char** argv) {
	setup_console();
	signal(SIGINT, sigint_handler);
	atexit(clean_up);
	return monotron_main();
}

int kbhit(void)
{
	struct termios oldt;
	int fd_stdin = fileno(stdin);
	tcgetattr(fd_stdin, &oldt);
	struct termios newt = oldt;
	newt.c_lflag &= ~(ICANON | ECHO);
	tcsetattr(fd_stdin, TCSANOW, &newt);
	int oldf = fcntl(fd_stdin, F_GETFL, 0);
	fcntl(fd_stdin, F_SETFL, oldf | O_NONBLOCK);
	int ch = getchar();
	tcsetattr(fd_stdin, TCSANOW, &oldt);
	fcntl(fd_stdin, F_SETFL, oldf);
	if(ch != EOF)
	{
		ungetc(ch, stdin);
		return 1;
	}
	return 0;
}

int getchar(void) {
	int ch = fgetc(stdin);
	return ch;
}

/* Write a connected sixel to the screen. Assumes you have the Teletext font selected. */
void put_connected_sixel(uint8_t ch) {
	if (ch == 0) {
		putchar(' ');
	} else {
		putchar('X');
	}
}

/* Write a separated sixel to the screen. Assumes you have the Teletext font selected. */
void put_separated_sixel(uint8_t ch) {
	if (ch == 0) {
		putchar(' ');
	} else {
		putchar('#');
	}
}

int putchar(int ch) {
	static bool have_escape = false;
	if (have_escape) {
		switch (ch) {
		case 'Z':
		case 'z':
			printf("\x1B[2J");
			break;
		case 'K':
			printf("\x1B[30m");
			break;
		case 'R':
			printf("\x1B[31m");
			break;
		case 'G':
			printf("\x1B[32m");
			break;
		case 'Y':
			printf("\x1B[33m");
			break;
		case 'B':
			printf("\x1B[34m");
			break;
		case 'M':
			printf("\x1B[35m");
			break;
		case 'C':
			printf("\x1B[36m");
			break;
		case 'W':
			printf("\x1B[37m");
			break;
		case 'k':
			printf("\x1B[40m");
			break;
		case 'r':
			printf("\x1B[41m");
			break;
		case 'g':
			printf("\x1B[42m");
			break;
		case 'y':
			printf("\x1B[43m");
			break;
		case 'b':
			printf("\x1B[44m");
			break;
		case 'm':
			printf("\x1B[45m");
			break;
		case 'c':
			printf("\x1B[46m");
			break;
		case 'w':
			printf("\x1B[47m");
			break;
		case '^':
		case 'v':
		case '-':
			// Ignore these codes (double height text)
			break;
		default:
			fprintf(stderr, "ERR: Unsupported code '%c' (%u)\n", ch, ch);
			abort();
			break;
		}
		have_escape = false;
	} else {
		if (ch == 27) {
			have_escape = true;
		} else {
			fputc(ch, stdout);
			fflush(stdout);
		}
	}
	return ch;
}

/* Write 8-bit string to stdout. */
int puts(const char* s) {
	while(*s) {
		putchar(*s++);
	}
	return 0;
}

/* Wait For Vertical Blanking Interval. */
void wfvbi(void) {
	struct timespec delay = {
		.tv_sec = 0,
		.tv_nsec = 1000000000 / 60
	};
	nanosleep(&delay, NULL);
}


void move_cursor(unsigned char row, unsigned char col) {
	printf("\x1B[%u;%uH", row + 1, col + 1);
}

int play(uint32_t frequency, channel_t channel, waveform_t waveform, uint8_t volume) {
	// TODO do audio thing here
	return 0;
}

/* Switch to the CodePage 850 font */
void font_normal(void) {
	// TODO how do we do this?
}

/* Switch to the Teletext font */
void font_teletext(void) {
	// TODO how do we do this?
}

/* Supply 4096 bytes of font data (16 bytes per char, 256 chars) */
void font_custom(const void* p_font) {
	// TODO how do we do this?
}

/* Fetch joystick state */
uint8_t get_joystick(void) {
	// TODO how do we do this?
	return 0;
}

/* Check joystick state */
bool joystick_is_up(uint8_t state) {
	return ((state & (1 << 4)) != 0);
}

bool joystick_is_down(uint8_t state) {
	return ((state & (1 << 3)) != 0);
}

bool joystick_is_left(uint8_t state) {
	return ((state & (1 << 2)) != 0);
}

bool joystick_is_right(uint8_t state) {
	return ((state & (1 << 1)) != 0);
}

bool joystick_fire_pressed(uint8_t state) {
	return ((state & (1 << 0)) != 0);
}

/* Enable/Disable cursor */
void set_cursor_visible(bool enabled) {
	// Disable cursor
	if (enabled) {
		printf("\x1B[?25h");
	} else {
		printf("\x1B[?25l");
	}
}

char * monotron_utoa(unsigned int value, char* str, int base)
{
  const char digits[] = "0123456789abcdefghijklmnopqrstuvwxyz";
  int i, j;
  unsigned remainder;
  char c;

  /* Check base is supported. */
  if ((base < 2) || (base > 36))
    {
      str[0] = '\0';
      return NULL;
    }

  /* Convert to string. Digits are in reverse order.  */
  i = 0;
  do
    {
      remainder = value % base;
      str[i++] = digits[remainder];
      value = value / base;
    } while (value != 0);
  str[i] = '\0';

  /* Reverse string.  */
  for (j = 0, i--; j < i; j++, i--)
    {
      c = str[j];
      str[j] = str[i];
      str[i] = c;
    }

  return str;
}

/* Disable cursor */
static void setup_console(void) {
	// Disable echo
	struct termios newt;
	int fd_stdin = fileno(stdin);
	tcgetattr(fd_stdin, &oldt);
	newt = oldt;
	newt.c_lflag &= ~(ICANON | ECHO);
	tcsetattr(fd_stdin, TCSANOW, &newt);
}

static void clean_up(void) {
	int fd_stdin = fileno(stdin);
	tcsetattr(fd_stdin, TCSANOW, &oldt);
	move_cursor(0, 0);
	printf("\x1B[?25h\x1B[0m\x1B[2J");
	exit(0);
}

static void sigint_handler(int dummy) {
	ungetc(0x03, stdin);
}
