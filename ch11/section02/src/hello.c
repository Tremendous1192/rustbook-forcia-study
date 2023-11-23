#include <stdio.h>

// C function to print "Hello, world from C!"
void c_hello() {
    printf("Hello, world from C!\n");
}

// C function to calculate the Fibonacci sequence
typedef unsigned int uint;

// Recursive implementation of Fibonacci sequence
uint c_fib(uint n) {
    if (n < 2) {
        return 1;
    } else {
        return c_fib(n - 1) + c_fib(n - 2);
    }
}
