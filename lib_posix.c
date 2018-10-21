#include <monotron.h>
#include <string.h>
#include <stdio.h>
#include <termios.h>
#include <time.h>
#include <fcntl.h>

int kbhit(void)
{
	struct termios oldt, newt;
	int fd_stdin = fileno(stdin);
	int ch;
	int oldf;
	tcgetattr(fd_stdin, &oldt);
	newt = oldt;
	newt.c_lflag &= ~(ICANON | ECHO);
	tcsetattr(fd_stdin, TCSANOW, &newt);
	oldf = fcntl(fd_stdin, F_GETFL, 0);
	fcntl(fd_stdin, F_SETFL, oldf | O_NONBLOCK);
	ch = getchar();
	tcsetattr(fd_stdin, TCSANOW, &oldt);
	fcntl(fd_stdin, F_SETFL, oldf);
	if(ch != EOF)
	{
		ungetc(ch, stdin);
		return 1;
	}
	return 0;
}

/* Write a connected sixel to the screen. Assumes you have the Teletext font selected. */
void put_connected_sixel(uint8_t ch) {
	ch = ch & 63;
	if (ch >= 32) {
		putchar(ch - 32 + 0xC0);
	} else {
		putchar(ch + 0x80);
	}
}

/* Write a separated sixel to the screen. Assumes you have the Teletext font selected. */
void put_separated_sixel(uint8_t ch) {
	ch = ch & 63;
	if (ch >= 32) {
		putchar(ch - 32 + 0xE0);
	} else {
		putchar(ch + 0xA0);
	}
}

int putchar(int ch) {
	static bool have_escape = false;
	if (have_escape) {
		switch (ch) {
		// TODO handle colours here
		default:
			// TODO unknown code?
			break;
		}
		have_escape = false;
	} else {
		if (ch == 27) {
			have_escape = true;
		} else {
			putc(ch, stdout);
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
	// TODO do ANSI thing here
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
