#include "../lib.h"

void swap(char *xp, char *yp)
{
	char temp = *xp;
	*xp = *yp;
	*yp = temp;
}

unsigned int partition(char *arr, unsigned int low, unsigned int high) {
    unsigned int pivot = arr[high];
    unsigned int i = (low - 1);

    for (unsigned int j = low; j <= (high - 1); j++) {
        if (arr[j] <= pivot) {
            i++;
            swap(&arr[i], &arr[j]);
        }
    }
    swap(&arr[i + 1], &arr[high]);
    return (i + 1);
}

void quick_sort(char *arr, unsigned int low, unsigned int high)
{
    if (low < high) {
        unsigned int i = partition(arr, low, high);

        quick_sort(arr, low, i - 1);
        quick_sort(arr, i + 1, high);
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
	quick_sort(arr, 0, len - 1);

	// Print output
	char *aft = "after: ";
	for (int i = 0; i < 7; i++) {
		print_char(aft[i]);
	}
	for (int i = 0; i < len; i++) {
		print_char(arr[i]);
	}
}
