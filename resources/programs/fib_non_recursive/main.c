// Should calculate fib(42) = 267914296
void _start()
{
	unsigned int a = 0;
	unsigned int b = 1;
	for (int i = 2; i <= 42; i++) {
		unsigned int c = b;
		b = a + b;
		a = c;
	}
	unsigned int ans = b;
}
