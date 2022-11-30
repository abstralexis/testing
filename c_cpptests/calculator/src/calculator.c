/*
This is my attempt at making a basic calculator in C
*/

#include <stdio.h>

#include "calculator.h"

int main(int argc, char **argv) {
    /*
    Main function
    */

    // Declare variables    
    int a = 10;
    int b = 100;

    // Get outputs from mathematical functions
    int output1 = add(a, b);
    int output2 = subtract(a, b);
    int output3 = multiply(a, b);
    int output4 = divide(b, a);

    // Print the outputs
    printf("The output is: %d\n", output1);
    printf("The output is: %d\n", output2);
    printf("The output is: %d\n", output3);
    printf("The output is: %d\n", output4);

    return 0;
}

int add(int a, int b) {
    /*
    Adds two numbers, a and b
    */
    return a + b;
}

int subtract(int a, int b) {
    /*
    Subtracts b from a
    */
    return a - b;
}

int multiply(int a, int b) {
    /*
    Multiplies two numbers, a and b
    */
    return a * b;
}

int divide(int a, int b) {
    /*
    Divides a by b
    */
    return a / b;
}