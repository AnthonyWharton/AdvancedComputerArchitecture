#include "lib.h"

void print_char(const char c) {
	asm("add a1,%0,0;"
		"ecall"
		:
		: "r" (c)
		:
	);
}

