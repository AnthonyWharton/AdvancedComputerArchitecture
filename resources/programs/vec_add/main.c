#include "../lib.h"

// Adds vec_a to vec_b, and prints the resulting surprise message!

int vec_a[21] = {32, 78, 99, 36, 12, 66, -34, 67, 39, 16, 73,  42, 10, 49, 32, 76, 17, 45, 141, -50, 13};
int vec_b[21] = {55, 33, 20,  8, 20, 52, 135, 32, 77, 95, 41, -10, 87, 51, 68, 29, 99, 60, -30, 160, 20};
int vec_r[21] = { 0,  0,  0,  0,  0,  0,   0,  0,  0,  0,  0,   0,  0,  0,  0,  0,  0,  0,   0,   0,  0};

void _start() {
	for (int i = 0; i < 21; i++) {
		vec_r[i] = vec_a[i] + vec_b[i];
	}
	for (int i = 0; i < 21; i++) {
		print_char(vec_r[i]);
	}
}
