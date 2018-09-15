#include <monotron.h>
#include <stdint.h>

struct callbacks_t {
	int32_t(*putchar)(void* p_context, char ch);
	int32_t(*puts)(void* p_context, const char*);
	int32_t(*readc)(void* p_context);
	void(*wfvbi)(void* p_context);
	void* p_context;
};

typedef int32_t(*entry_point_t)(const struct callbacks_t*);

const struct callbacks_t* p_callbacks;
void* p_context;

static unsigned int rand_seed = 0;

int32_t entry(const struct callbacks_t* callbacks) {
	p_callbacks = callbacks;
	return main();
}

/* Write 8-bit char to stdout */
int putchar(char ch) {
	return p_callbacks->putchar(p_callbacks->p_context, ch);
}

/* Write 8-bit string to stdout. */
int puts(const char* s) {
	return p_callbacks->puts(p_callbacks->p_context, s);
}

/* Blocking character read from stdin. Returns a char or EOF */
int getchar(void) {
	return p_callbacks->readc(p_callbacks->p_context);
}

/* Wait For Vertical Blanking Interval. */
void wfvbi(void) {
	p_callbacks->wfvbi(p_callbacks->p_context);
}

int rand(void) {
	rand_seed = (rand_seed * 1103515245) + 12345;
	return rand_seed;
}

void srand(unsigned int seed) {
	rand_seed = seed;
}

__attribute__ ((section(".entry_point")))
const entry_point_t entry_point = entry;
