/*
 *  For some examples, there will be a C and C++ example, but 
 *  lots of the time, it would just be unnecessary.
 */

#include <stdio.h>
#include <stdbool.h>        // Bools are a standard language feature in C++

int main(int argc, char **argv) {
    // Basic int stuff
    int x = 10;
    int y = 20;
    int result = x + y;
    printf("%d\n", result); // Format outputs with %d. Note the \n is needed.
    long bigInt;            // Longs store a double length integer

    // Floating point types
    float decimalPoint = 0.0;
    double preciseDecimal;
    long double reallyPreciceDecimal;

    // Characters and character arrays (a.k.a. strings)
    char *character = "c\n";                  // A char pointer. This just illustrates a use case.
    printf(character);
    char characters[12] = "Hello World\n";
    printf(characters);

    // Using bool from stdbool.h
    bool isCCool = true;
    if (isCCool) {
        printf("C is cool!\n");
    }

    // Pointers will be covered properly in another file.

    return 0;
}