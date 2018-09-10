#include <monotron.h>

int main(const struct callbacks_t* p_callbacks) {
	p_callbacks->puts("Hello from RAM!\n");
	return 1;
}
