/***********************************************************************************
 *
 *  Snake game for Monotron homebrew computer. Based on Snake for the RC2014.
 *  See https://github.com/RC2014Z80/RC2014/blob/master/ROMs/snake/snake.c
 *
 *  The MIT License (MIT)
 *  Copyright (c) 2016 Filippo Bergamasco
 *  Copyright (c) 2018 Jonathan 'theJPster' Pallant
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
 * of the Software, and to permit persons to whom the Software is furnished to do
 * so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 **********************************************************************************/

#ifdef PC_BUILD
// Build for Linux
#include <stdio.h>
#include <time.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>
#include <termios.h>
#include <unistd.h>
#include <fcntl.h>
int kbhit(void) {
    struct termios oldt, newt;
    int ch;
    int oldf;
    tcgetattr(STDIN_FILENO, &oldt);
    newt = oldt;
    newt.c_lflag &= ~(ICANON | ECHO);
    tcsetattr(STDIN_FILENO, TCSANOW, &newt);
    oldf = fcntl(STDIN_FILENO, F_GETFL, 0);
    fcntl(STDIN_FILENO, F_SETFL, oldf | O_NONBLOCK);
    ch = getchar();
    tcsetattr(STDIN_FILENO, TCSANOW, &oldt);
    fcntl(STDIN_FILENO, F_SETFL, oldf);
    if(ch != EOF) {
        ungetc(ch, stdin);
        return 1;
    }
    return 0;
}

#define CHANNEL_0 0
#define WAVEFORM_SQUARE 0
#define MAX_VOLUME 255
int play(uint32_t frequency, uint8_t channel, uint8_t waveform, uint8_t volume) {
    return 0;
}
#else
// Use the Monotron API
#include <monotron.h>
#endif

#define FIELD_CHAR 'F'
#define APPLE_CHAR 'A'
#define UP_CHAR 'U'
#define DOWN_CHAR 'D'
#define LEFT_CHAR 'L'
#define RIGHT_CHAR 'R'

#ifdef PC_BUILD
#define SNAKE_COLOR 48
#define FIELD_COLOR 248
#define BG_COLOR 0
#define APPLE_COLOR 160
#else
#define SNAKE_COLOR 6
#define FIELD_COLOR 3
#define BG_COLOR 7
#define APPLE_COLOR 1
#endif

#define FIELD_W 46
#define FIELD_H 33
#define SCORE_PER_APPLE 10

struct {
    int i;
    int j;
} snake_head;

struct {
    int i;
    int j;
} snake_tail;

static unsigned char field[FIELD_W * FIELD_H];
static unsigned int score;
static unsigned int rnd_x = 4;
static unsigned int rnd_y = 113;
static unsigned int rnd_z = 543;
static unsigned int rnd_w = 11;

static unsigned int sound_frames_remaining = 0;

static void pigfx_bgcol(unsigned int color);
static void pigfx_cls(void);
static void pigfx_fgcol(unsigned int color);
static void pigfx_hide_cursor(void);
static void pigfx_movecursor(unsigned int row, unsigned int col);
static void pigfx_print(const char* s);
static void pigfx_printnum(unsigned int num);
#ifdef PC_BUILD
static void wfvbi(void);
#endif
static unsigned int xorshift128(void);
static void new_apple(void);
static void update_score(unsigned int score);
static void initialize(void);
static int update_snake(void);
static void beep(uint32_t frequency, uint8_t frames, uint8_t volume);
static void wait_frame(void);
static void wait_note(void);
static void update_sound(void);

static void pigfx_bgcol(unsigned int color) {
#ifdef PC_BUILD
    printf("\x1b[48;5;%um", color);
#else
    const char colours[] = "wrgbcmyk";
    putchar(27);
    putchar(colours[color]);
#endif
}

static void pigfx_cls(void) {
#ifdef PC_BUILD
    printf("\x1b[2J");
#else
    putchar(27);
    putchar('Z');
#endif
}

static void pigfx_fgcol(unsigned int color) {
#ifdef PC_BUILD
    printf("\x1b[38;5;%um", color);
#else
    const char colours[] = "WRGBCMYK";
    putchar(27);
    putchar(colours[color]);
#endif
}

static void pigfx_hide_cursor(void) {
#ifdef PC_BUILD
    printf("\x1b[?25l");
#endif
}

static void pigfx_movecursor(unsigned int row, unsigned int col) {
#ifdef PC_BUILD
    printf("\x1b[%u;%uH", row, col);
#else
    move_cursor(row, col);
#endif
}

static void pigfx_print(const char* s) {
#ifdef PC_BUILD
    printf("%s", s);
#else
    puts(s);
#endif
}

static void pigfx_printnum(unsigned int num) {
#ifdef PC_BUILD
    printf("%u", num);
#else
    // 4294967296 is the largest we can print
    char buffer[12] = { 0 };
    itoa(num, buffer);
    puts(buffer);
#endif
}

#ifdef PC_BUILD
static void wfvbi(void) {
    struct timespec amount = {
        .tv_sec = 0,
        .tv_nsec = 1000 * 1000 * (1000 / 60)
    };
    nanosleep(&amount, 0);
}
#endif

static unsigned int xorshift128(void) {
    unsigned int t = rnd_x;
    t ^= t << 11;
    t ^= t >> 8;
    rnd_x = rnd_y;
    rnd_y = rnd_z;
    rnd_z = rnd_w;
    rnd_w ^= rnd_w >> 19;
    rnd_w ^= t;
    return rnd_w;
}

static void new_apple(void) {
    unsigned int apple_i;
    unsigned int apple_j;
    unsigned int apple_idx;

    while (1) {
        apple_i = (xorshift128() % (FIELD_H - 3)) + 2;
        apple_j = (xorshift128() % (FIELD_H - 3)) + 2;

        apple_idx = apple_i * FIELD_W + apple_j;

        if (field[apple_idx] == 0) {
            field[apple_idx] = APPLE_CHAR;
            pigfx_movecursor(apple_i + 1, apple_j + 1);
            pigfx_fgcol(APPLE_COLOR);
            pigfx_bgcol(BG_COLOR);
            putchar('@');
            pigfx_fgcol(SNAKE_COLOR);
            return;
        }
    }
}

static void update_score(unsigned int score) {
    pigfx_print("SCORE: ");
    pigfx_printnum(score);
}

static void initialize(void) {
    int i;
    int j;
    int head_idx;
    unsigned char* pfield = field;

    score = 0;

    pigfx_cls();
    pigfx_hide_cursor();

    pigfx_bgcol(FIELD_COLOR);
    // Top
    pigfx_movecursor(1, 1);
    pfield = field;

    for (i = 0; i < FIELD_W; ++i) {
        putchar(' ');
        *pfield++ = FIELD_CHAR;
    }

    // Left-Right
    for (i = 1; i < FIELD_H - 1; ++i) {
        pigfx_bgcol(FIELD_COLOR);
        pigfx_movecursor(i + 1, 1);

        putchar(' ');
        *pfield++ = FIELD_CHAR;

        pigfx_bgcol(BG_COLOR);
        for (j = 1; j < FIELD_W - 1; ++j) {
            putchar(' ');
            *pfield++ = 0;
        }

        pigfx_bgcol(FIELD_COLOR);
        putchar(' ');
        *pfield++ = FIELD_CHAR;
    }

    // Bottom
    pigfx_movecursor(FIELD_H, 1);
    for (i = 0; i < FIELD_W; ++i) {
        putchar(' ');
        *pfield++ = FIELD_CHAR;
    }

    // Snake
    pigfx_bgcol(SNAKE_COLOR);

    snake_head.i = FIELD_H / 2;
    snake_head.j = FIELD_W / 2;
    snake_tail.i = snake_head.i + 2;
    snake_tail.j = snake_head.j;
    head_idx = snake_head.i * FIELD_W + snake_head.j;
    field[head_idx] = UP_CHAR;
    field[head_idx+FIELD_W] = UP_CHAR;
    field[head_idx+FIELD_W+FIELD_W] = UP_CHAR;

    pigfx_movecursor(snake_head.i + 1, snake_head.j + 1);
    putchar(' ');
    pigfx_movecursor(snake_head.i + 2, snake_head.j + 1);
    putchar(' ');
    pigfx_movecursor(snake_head.i + 3, snake_head.j + 1);
    putchar(' ');

    new_apple();

    // Credits/Help
    pigfx_bgcol(BG_COLOR);
    pigfx_movecursor(FIELD_H + 1, 1);
    pigfx_fgcol(SNAKE_COLOR);
    pigfx_print("*SNAKE* ");
    pigfx_fgcol(15);
    pigfx_print("F.Bergamasco 2016");
    pigfx_movecursor(FIELD_H + 2, 1);
    pigfx_print(" w:up s:down a:left d:right n:new game p:pause");
    pigfx_movecursor(FIELD_H + 1, 34);
    update_score(score);
}

static int update_snake(void) {
    int head_idx = snake_head.i * FIELD_W + snake_head.j;
    int tail_idx = snake_tail.i * FIELD_W + snake_tail.j;
    unsigned char c_head = field[head_idx];
    unsigned char keepsize = 1;

    switch(c_head) {
    case UP_CHAR:
        head_idx -= FIELD_W;
        snake_head.i--;
        break;

    case DOWN_CHAR:
        head_idx += FIELD_W;
        snake_head.i++;
        break;

    case LEFT_CHAR:
        head_idx--;
        snake_head.j--;
        break;

    case RIGHT_CHAR:
        head_idx++;
        snake_head.j++;
        break;
    }

    if (field[head_idx] == APPLE_CHAR) {
        keepsize = 0;
        score += SCORE_PER_APPLE;
        pigfx_bgcol(BG_COLOR);
        pigfx_movecursor(FIELD_H + 1, 34);
        update_score(score);
        beep(1000, 5, MAX_VOLUME);
    } else {
        if (field[head_idx] != 0) {
            return 0;
        }
    }

    pigfx_bgcol(SNAKE_COLOR);
    field[head_idx] = c_head;
    pigfx_movecursor(snake_head.i + 1, snake_head.j + 1);
    putchar(' ');

    if (keepsize) {
        c_head = field[tail_idx];
        field[tail_idx] = 0;
        pigfx_bgcol(BG_COLOR);
        pigfx_movecursor(snake_tail.i + 1, snake_tail.j + 1);
        putchar(' ');

        switch(c_head) {
        case UP_CHAR:
            snake_tail.i--;
            break;

        case DOWN_CHAR:
            snake_tail.i++;
            break;

        case LEFT_CHAR:
            snake_tail.j--;
            break;

        case RIGHT_CHAR:
            snake_tail.j++;
            break;
        }
    } else {
        new_apple();
    }

    return 1;
}

// Start a beep on Channel 0 (but don't wait for it to finish)
static void beep(uint32_t frequency, uint8_t frames, uint8_t volume) {
    play(frequency * 100, CHANNEL_0, WAVEFORM_SQUARE, volume);
    sound_frames_remaining = frames;
}

// Wait for next frame (updating sound as required)
static void wait_frame(void) {
    wfvbi();
    update_sound();
}

// Wait for current note to finish
static void wait_note(void) {
    while (sound_frames_remaining != 0) {
        wfvbi();
        update_sound();
    }
}

// Stop the sound when we've heard enough
static void update_sound(void) {
    if (sound_frames_remaining > 0) {
        sound_frames_remaining--;
        if (sound_frames_remaining == 0) {
            play(0, CHANNEL_0, WAVEFORM_SQUARE, 0);
        }
    }
}

int main(void) {
    char usercommand;
    int  head_idx;

    initialize();

    while (!kbhit() || getchar() != 'n')
        rnd_x++;

    while (1) {
        if (update_snake() == 0) {
            pigfx_movecursor(FIELD_H / 2, FIELD_W / 2 - 5);
            pigfx_print("GAME OVER!");
            beep(880, 30, MAX_VOLUME);
            wait_note();
            beep(440, 60, MAX_VOLUME);
            wait_note();
            while (getchar() != 'n') {
                wait_frame();
            };
            initialize();
            continue;
        }

        usercommand = 0; // none

        if (kbhit()) {
            usercommand = getchar();
        }

        head_idx = snake_head.i * FIELD_W + snake_head.j;

        switch(usercommand) {
        case 'w':
        case 'W':
            if (field[head_idx] != DOWN_CHAR)
                field[head_idx] = UP_CHAR;
            break;

        case 'd':
        case 'D':
            if (field[head_idx] != LEFT_CHAR)
                field[head_idx] = RIGHT_CHAR;
            break;

        case 'a':
        case 'A':
            if (field[head_idx] != RIGHT_CHAR)
                field[head_idx] = LEFT_CHAR;
            break;

        case 's':
        case 'S':
            if (field[head_idx] != UP_CHAR)
                field[head_idx] = DOWN_CHAR;
            break;

        case 'n':
        case 'N':
            initialize();
            continue;

        case 'p':
        case 'P':
            while (1) {
                if (kbhit() && getchar() == 'p') {
                    break;
                }
                wait_frame();
            }
            break;

        default:
            // do nothing
            break;
        }

        // Only bip if nothing else playing
        if (sound_frames_remaining == 0) {
            beep(1000, 2, MAX_VOLUME / 4);
        }

        // DELAY LOOP - runs at 10 frames per second
        for(int i = 0; i < 6; i++) {
            wait_frame();
        }
    }
}
