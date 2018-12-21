#include "../lib.h"

void swap(char *xp, char *yp)
{
	char temp = *xp;
	*xp = *yp;
	*yp = temp;
}

void bubble_sort(char *arr, int n)
{
	for (int i = 0; i < n-1; i++) {
		// Last i elements are already in place
		for (int j = 0; j < n-i-1; j++) {
			if (arr[j] > arr[j+1]) {
				swap(&arr[j], &arr[j+1]);
			}
		}
	}
}

void _start()
{
	unsigned int len = 8;
	char *arr = "daybreak";

	char *bef = "before: ";
	for (int i = 0; i < 8; i++) {
		print_char(bef[i]);
	}
	for (int i = 0; i < len; i++) {
		print_char(arr[i]);
	}
	print_char('\n');

	// Sort!
	bubble_sort(arr, len);

	// Print output
	char *aft = "after: ";
	for (int i = 0; i < 7; i++) {
		print_char(aft[i]);
	}
	for (int i = 0; i < len; i++) {
		print_char(arr[i]);
	}
}
