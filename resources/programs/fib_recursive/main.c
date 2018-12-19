unsigned int fib(unsigned int n) {
	if (n == 0) {
		return 0;
	} else if (n <= 2) {
		return 1;
	} else {
		return fib(n-1) + fib(n-2);
	}
}

void _start()
{
	unsigned int result = fib(9);
}
