#include "../lib.h"

const int len = 12;
const char *hello = "hello world!";

void _start() {
	for (int i = 0; i < len; i++) {
		print_char(hello[i]);
	}
}
