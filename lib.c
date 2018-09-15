#include <monotron.h>

const struct callbacks_t* p_callbacks;

int entry(const struct callbacks_t* p) {
	p_callbacks = p;
	return main();
}

int putchar(char ch) {
	return p_callbacks->putchar(ch);
}

int puts(const char* s) {
	return p_callbacks->puts(s);
}

__attribute__ ((section(".entry_point")))
static const entry_point_t entry_point = entry;
