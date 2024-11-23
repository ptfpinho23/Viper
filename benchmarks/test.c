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

    return 0;
}
