static int lol = 0x42;

int test_function() {
	int a = 3;
	int b = 5;
	return a * b;	
}

void _start() {
	int ans = test_function();
}
