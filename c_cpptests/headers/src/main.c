#include <stdio.h>  // This is how to include a header.
#include "header.h" // This is how we include our custom header

int main(int argc, char *argv[]) {
    float x = 10.6;
    float y = f(x);                 // Use the function from the header
    printf("%f", y);                // Print the result
    /*
    We gotta love computers being bad at maths, especially with floats.
    */    
}