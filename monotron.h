#include <stdint.h>

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

#define NULL ((void*) 0)
#define RAND_MAX 32768
#define FRAMES_PER_SECOND 60
