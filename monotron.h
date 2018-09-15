typedef int(*puts_t)(const char*);
typedef int(*putc_t)(char ch);
struct callbacks_t {
	putc_t putchar;
	puts_t puts;
};
typedef int(*entry_point_t)(const struct callbacks_t*);

int main(void);
int putchar(char ch);
int puts(const char* s);
