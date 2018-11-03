void _start()
{
	volatile int a = 0;
	int b = 1;
	for (int i = 3; i <= 8; i++) {
		int c = b;
		b = a + b;
		a = c;
	}
}
