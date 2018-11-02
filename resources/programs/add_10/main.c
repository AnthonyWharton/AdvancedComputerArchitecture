void _start() {
	volatile int foo = 0;
	for (int i = 0; i < 10; i++) {
		foo += i;
	}
}
