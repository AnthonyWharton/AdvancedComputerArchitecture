unsigned int fib(unsigned int n)
{
	unsigned int a = 0;
	unsigned int b = 1;
	for (int i = 2; i <= n; i++) {
		unsigned int c = b;
		b = a + b;
		a = c;
	}
	return b;
}

void _start()
{
	volatile unsigned int foo = fib(9);
}
