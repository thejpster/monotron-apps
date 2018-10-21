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

#include <monotron.h>
#include <string.h>
#include <stdlib.h>

#define FIELD_CHAR 'F'
#define APPLE_CHAR 'A'
#define UP_CHAR 'U'
#define DOWN_CHAR 'D'
#define LEFT_CHAR 'L'
#define RIGHT_CHAR 'R'

#define SNAKE_COLOR 6
#define FIELD_COLOR 3
#define BG_COLOR 7
#define APPLE_COLOR 1
#define HISCORE_COLOR 1

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

typedef struct music_event_t {
    uint32_t frame;
    uint32_t frequency_centihz;
    uint8_t volume;
    waveform_t waveform;
} music_event_t;

typedef struct track_t {
    channel_t channel;
    const music_event_t* p_events;
    size_t num_events;
    size_t current_event;
    uint32_t wrap_frame_at;
    uint32_t current_frame;
} track_t;

static const music_event_t TRACK0_EVENTS[] = {
    { 0, Note_C2, 100, WAVEFORM_SAWTOOTH },
    { 7, Note_E2, 100, WAVEFORM_SAWTOOTH },
    { 15, Note_G2, 100, WAVEFORM_SAWTOOTH },
    { 22, Note_C3, 100, WAVEFORM_SAWTOOTH },
};

static track_t TRACK0 = {
    .channel = CHANNEL_0,
    .p_events = TRACK0_EVENTS,
    .num_events = ELEMOF(TRACK0_EVENTS),
    .current_event = 0,
    .wrap_frame_at = 30,
    .current_frame = 0,
};

static const music_event_t TRACK1_EVENTS[] = {
    { 30, Note_C1, 255, WAVEFORM_NOISE },
    { 32, 0, 0, WAVEFORM_NOISE },
    { 75, Note_C1, 255, WAVEFORM_NOISE },
    { 77, 0, 0, WAVEFORM_NOISE },
    { 90, Note_C1, 255, WAVEFORM_NOISE },
    { 92, 0, 0, WAVEFORM_NOISE },
};

static track_t TRACK1 = {
    .channel = CHANNEL_1,
    .p_events = TRACK1_EVENTS,
    .num_events = ELEMOF(TRACK1_EVENTS),
    .current_event = 0,
    .wrap_frame_at = 120,
    .current_frame = 0,
};

static const music_event_t TRACK2_EVENTS[] = {
    { 0, Note_C5, 96, WAVEFORM_SQUARE },
    { 0 + 40, 0, 0, WAVEFORM_SQUARE },
    { 45, Note_A4, 96, WAVEFORM_SQUARE },
    { 45 + 10, 0, 0, WAVEFORM_SQUARE },
    { 60, Note_G4, 96, WAVEFORM_SQUARE },
    { 60 + 15, 0, 0, WAVEFORM_SQUARE },
    { 90, Note_F4, 96, WAVEFORM_SQUARE },
    { 90 + 15, 0, 0, WAVEFORM_SQUARE },
    { 120, Note_G4, 96, WAVEFORM_SQUARE },
    { 120 + 15, 0, 0, WAVEFORM_SQUARE },
    { 150, Note_F4, 96, WAVEFORM_SQUARE },
    { 150 + 15, 0, 0, WAVEFORM_SQUARE },
    { 180, Note_G4, 96, WAVEFORM_SQUARE },
    { 180 + 15, 0, 0, WAVEFORM_SQUARE },
    { 210, Note_B4, 96, WAVEFORM_SQUARE },
    { 210 + 15, 0, 0, WAVEFORM_SQUARE },
    { 240, Note_C5, 96, WAVEFORM_SQUARE },
    { 240 + 40, 0, 0, WAVEFORM_SQUARE },
    { 285, Note_A4, 96, WAVEFORM_SQUARE },
    { 285 + 10, 0, 0, WAVEFORM_SQUARE },
    { 300, Note_G4, 96, WAVEFORM_SQUARE },
    { 300 + 15, 0, 0, WAVEFORM_SQUARE },
    { 330, Note_F4, 96, WAVEFORM_SQUARE },
    { 330 + 15, 0, 0, WAVEFORM_SQUARE },
    { 360, Note_G4, 96, WAVEFORM_SQUARE },
    { 360 + 15, 0, 0, WAVEFORM_SQUARE },
    { 390, Note_B4, 96, WAVEFORM_SQUARE },
    { 390 + 15, 0, 0, WAVEFORM_SQUARE },
    { 420, Note_C5, 96, WAVEFORM_SQUARE },
    { 420 + 45, 0, 0, WAVEFORM_SQUARE },
};

static track_t TRACK2 = {
    .channel = CHANNEL_2,
    .p_events = TRACK2_EVENTS,
    .num_events = ELEMOF(TRACK2_EVENTS),
    .current_event = 0,
    .wrap_frame_at = 480,
    .current_frame = 0,
};

static bool music_playing = false;
static unsigned char field[FIELD_W * FIELD_H];
static unsigned int score;
static unsigned int hiscore;
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
static unsigned int xorshift128(void);
static void new_apple(void);
static void update_score(unsigned int score);
static void splash_screen(void);
static void initialize(void);
static int update_snake(void);
static void beep(uint32_t frequency, uint8_t frames, uint8_t volume);
static void wait_frame(void);
static void wait_note(void);
static void update_sound(void);
static char get_input(void);
static void game(void);

static void pigfx_bgcol(unsigned int color) {
    const char colours[] = "wrgbcmyk";
    putchar(27);
    putchar(colours[color]);
}

static void pigfx_cls(void) {
    putchar(27);
    putchar('Z');
}

static void pigfx_fgcol(unsigned int color) {
    const char colours[] = "WRGBCMYK";
    putchar(27);
    putchar(colours[color]);
}

static void pigfx_hide_cursor(void) {

}

static void pigfx_movecursor(unsigned int row, unsigned int col) {
    move_cursor(row, col);
}

static void pigfx_print(const char* s) {
    puts(s);
}

static void pigfx_printnum(unsigned int num) {
    // 4294967296 is the largest we can print
    char buffer[12] = { 0 };
    monotron_utoa(num, buffer, 10);
    puts(buffer);
}

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

static void splash_screen(void) {
    font_teletext();
    // Clear the screen (white on black)
    puts("\ek\eW\eZ\eG\n\n");

    // Draw the logo
    static const uint8_t sixels[]  = {
        0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  0,  0,  48, 60, 63, 21, 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  48, 60, 63, 63, 60, 52, 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  0,  0,  3,  43, 63, 21, 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  56, 63, 7,  0,  0,  47, 63, 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  0,  0,  0,  42, 63, 21, 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  63, 63, 0,  0,  0,  42, 63, 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  0,  0,  0,  42, 63, 21, 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  63, 63, 16, 0,  0,  2,  3,  0,  32, 60, 63, 21, 56, 62, 63, 60, 0,  0,  0,  32, 60, 63,
        63, 63, 52, 0,  0,  0,  42, 63, 21, 0,  40, 60, 60, 20, 0,  0,  32, 60, 63, 63, 60, 16, 0,  0,
        0,  0,  63, 63, 63, 52, 0,  0,  0,  0,  2, 43,  63, 31, 3,  3,  63, 63, 20, 0,  0,  63, 63, 0,
        0,  43, 63, 21, 0,  0,  42, 63, 21, 0,  32, 63, 1,  0,  0,  32, 63, 23, 0,  2,  63, 53, 0,  0,
        0,  0,  2,  63, 63, 63, 63, 52, 0,  0,  0,  42, 63, 21, 0,  0,  43, 63, 21, 0,  0,  15, 15, 0,
        0,  42, 63, 21, 0,  0,  42, 63, 21, 32, 62, 5,  0,  0,  0,  42, 63, 0,  0,  0,  42, 63, 20, 0,
        0,  0,  0,  2,  15, 63, 63, 63, 52, 0,  0,  42, 63, 21, 0,  0,  42, 63, 21, 0,  0,  0,  0,  48,
        56, 62, 63, 21, 0,  0,  42, 63, 21, 62, 53, 0,  0,  0,  0,  63, 63, 63, 63, 63, 63, 63, 21, 0,
        0,  0,  0,  0,  0,  2,  63, 63, 63, 0,  0,  42, 63, 21, 0,  0,  42, 63, 21, 0,  0,  56, 63, 15,
        3,  43, 63, 21, 0,  0,  42, 63, 31, 47, 63, 20, 0,  0,  0,  63, 63, 0,  0,  0,  0,  0,  0,  0,
        0,  40, 63, 0,  0,  0,  2,  63, 63, 0,  0,  42, 63, 21, 0,  0,  42, 63, 21, 0,  40, 63, 23, 0,
        0,  42, 63, 21, 0,  0,  42, 63, 21, 10, 63, 61, 0,  0,  0,  43, 63, 20, 0,  0,  0,  0,  0,  0,
        0,  42, 63, 0,  0,  0,  32, 63, 63, 0,  0,  42, 63, 21, 0,  0,  42, 63, 21, 0,  42, 63, 53, 0,
        0,  42, 63, 21, 0,  0,  42, 63, 21, 0,  43, 63, 21, 0,  0,  42, 63, 61, 16, 0,  32, 56, 20, 0,
        0,  42, 63, 52, 48, 48, 63, 63, 1,  0,  32, 62, 63, 61, 16, 32, 62, 63, 53, 16, 0,  63, 63, 60,
        0,  47, 63, 53, 16, 32, 62, 63, 53, 16, 2,  63, 63, 52, 0,  0,  11, 63, 63, 63, 63, 31, 1,  0,
        0,  0,  3,  15, 15, 15, 3,  0,  0,  0,  2,  3,  3,  3,  1,  2,  3,  3,  3,  1,  0,  0,  3,  3,
        0,  2,  3,  3,  1,  2,  3,  3,  3,  1,  0,  3,  3,  3,  0,  0,  0,  2,  3,  3,  1,  0,  0,  0
    };
    for(size_t i = 0; i < sizeof(sixels); i++) {
        put_separated_sixel(sixels[i]);
    }
    puts(
        "\n\n\n\eY"
        "Original for RC2014, Copyright F.Bergamasco 2016\n"
        "Monotron version, Copyright J.Pallant 2018\n"
        "\eC\n"
        "Press 'p' or Fire to start..."
    );

    // Configure music here
}

static void initialize(void) {
    int i;
    int j;
    int head_idx;
    unsigned char* pfield = field;

    font_normal();

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
    pigfx_print(" w:up s:down a:left d:right p:pause/start");
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

static void play_note(track_t* p_track) {
    if (p_track->current_frame == p_track->wrap_frame_at) {
        p_track->current_frame = 0;
        p_track->current_event = 0;
    }
    const music_event_t* p_event = &p_track->p_events[p_track->current_event];
    if (p_track->current_event < p_track->num_events) {
        if (p_track->current_frame >= p_event->frame) {
            // Play note
            play(p_event->frequency_centihz, p_track->channel, p_event->waveform, p_event->volume);
            // Move to next event
            p_track->current_event++;
        }
    }
    p_track->current_frame += 1;
}

// Stop the sound when we've heard enough
static void update_sound(void) {
    if (music_playing) {
        // Play music here
        play_note(&TRACK0);
        play_note(&TRACK1);
        play_note(&TRACK2);
    } else if (sound_frames_remaining > 0) {
        sound_frames_remaining--;
        if (sound_frames_remaining == 0) {
            play(0, CHANNEL_0, WAVEFORM_SQUARE, 0);
        }
    }

}

static char get_input(void) {
    char usercommand = 0;
    if (kbhit()) {
        usercommand = getchar();
    } else {
        uint8_t js = get_joystick();
        if (joystick_fire_pressed(js)) {
            usercommand = 'p';
        } else if (joystick_is_up(js)) {
            usercommand = 'w';
        } else if (joystick_is_down(js)) {
            usercommand = 's';
        } else if (joystick_is_left(js)) {
            usercommand = 'a';
        } else if (joystick_is_right(js)) {
            usercommand = 'd';
        }
    }
    return usercommand;
}

static void start_music(void) {
    music_playing = true;
    TRACK0.current_frame = 0;
    TRACK0.current_event = 0;
    TRACK1.current_frame = 0;
    TRACK1.current_event = 0;
    TRACK2.current_frame = 0;
    TRACK2.current_event = 0;
}

static void game_over(void) {
    pigfx_movecursor(FIELD_H / 2, (FIELD_W - 10) / 2);
    pigfx_print("GAME OVER!");
    pigfx_movecursor((FIELD_H / 2) + 1, (FIELD_W - 31) / 2);
    pigfx_print("Press 'p' or Fire to try again.");
    beep(880, 30, MAX_VOLUME);
    wait_note();
    beep(440, 60, MAX_VOLUME);
    wait_note();

    // Handle high score stuff.

    int new_hi_score = 0;
    if (score > hiscore) {
        hiscore = score;
        new_hi_score = 1;
    }
    pigfx_movecursor((FIELD_H / 2) + 4, (FIELD_W - 14) / 2);
    pigfx_print("HI SCORE: ");

    pigfx_fgcol(APPLE_COLOR);
    pigfx_printnum(hiscore);
    if (new_hi_score) {
        pigfx_movecursor((FIELD_H / 2) + 5, (FIELD_W - 14) / 2);
        pigfx_print("NEW HI SCORE!");
    }

    // Wait for keypress
    for (char c = get_input(); (c != 'p') && (c != 'P'); c = get_input()) {
        wait_frame();
    }
}

static void game(void) {
    int  head_idx;
    start_music();
    splash_screen();

    // Don't immediately exit the splash screen because the button is still held.
    for(int i = 0; i < 30; i++) {
        wait_frame();
    }

    for (char c = get_input(); (c != 'p') && (c != 'P'); c = get_input()) {
        wait_frame();
        rnd_x++;
    }

    // Stop the music
    play(0, CHANNEL_0, WAVEFORM_SQUARE, 0);
    play(0, CHANNEL_1, WAVEFORM_SQUARE, 0);
    play(0, CHANNEL_2, WAVEFORM_SQUARE, 0);
    music_playing = false;

    initialize();

    while (1) {
        if (update_snake() == 0) {
            game_over();
            return;
        }

        head_idx = snake_head.i * FIELD_W + snake_head.j;

        switch(get_input()) {
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

        case 'p':
        case 'P':
            for (char c = get_input(); (c != 'p') && (c != 'P'); c = get_input()) {
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

int main(void) {
    while (1) {
        game();
    }
}
