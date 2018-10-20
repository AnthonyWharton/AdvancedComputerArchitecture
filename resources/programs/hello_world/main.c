static int lol = 0x42;

int test_function() {
	int a = 3;
	int b = 5;
	return a * b;	
}

void _start() {
	int ans = test_function();
	// const int a[5] = {1, 2, 3, 4, 5};
	// const int b[5] = {1, 2, 3, 4, 5};
	// int c[5] = {0, 0, 0, 0, 0};
	
	// for (int i = 0; i < 5; i++) {
	// 	c[i] = a[i] * b[i];
	// }
}
