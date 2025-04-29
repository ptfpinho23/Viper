#include <stdio.h>

int main() {
    // Perform calculations
    long long x = 10 / 2;
    long long d;
    if (x == 4) {
        d = 50 + 3;
    } else {
        d = 50 - 1;
    }
    printf("%lld\n", d);
    // Print results
    printf("%lld\n", x);

    // Add while loop examples
    long long i = 1;
    while (i == 1) {
        printf("100\n");
        i = 0;
    }

    long long counter = 3;
    while (counter > 0) {
        printf("%lld\n", counter);
        counter = counter - 1;
    }

    return 0;
}
