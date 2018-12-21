void print(const char c) {
	asm("add a1,%0,0;"
		"ecall"
		:
		: "r" (c)
		:
	);
}

const int len = 14;
const char *hello = "hello\n  world!";

void _start() {
	// print('y');
	for (int i = 0; i < len; i++) {
		print(hello[i]);
	}
}
