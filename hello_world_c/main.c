typedef int(*puts_t)(const char*);
typedef int(*putc_t)(int ch);
struct callbacks_t {
	putc_t putc;
	puts_t puts;
};
typedef int(*entry_point_t)(const struct callbacks_t*);

static int main(const struct callbacks_t* p_callbacks);

__attribute__ ((section(".entry_point")))
static const entry_point_t entry_point = main;

static int main(const struct callbacks_t* p_callbacks) {
	p_callbacks->puts("Hello from RAM!\n");
	return 1;
}
