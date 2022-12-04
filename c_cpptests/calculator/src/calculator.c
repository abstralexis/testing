/*
This is my attempt at making a basic calculator in C
*/

#include <stdio.h>
#include <stdlib.h>

#include "calculator.h"

int main(int argc, char **argv) {
    /*
    Main function
    */

    // Declare variables    
    long int a = 10;
    long int b = 100;

    // Get outputs from mathematical functions
    long int output1 = add(a, b);
    long int output2 = subtract(a, b);
    long int output3 = multiply(a, b);
    long int output4 = divide(b, a);

    // Print the outputs
    printf("The output is: %d\n", output1);
    printf("The output is: %d\n", output2);
    printf("The output is: %d\n", output3);
    printf("The output is: %d\n", output4);

    printf("\nInput an integer operand: ");
    long int operandTest = getOperand();
    printf("The operand is: %d\n", operandTest);

    return 0;
}

long int add(long int a, long int b) {
    /*
    Adds two numbers, a and b
    */
    return a + b;
}

long int subtract(long int a, long int b) {
    /*
    Subtracts b from a
    */
    return a - b;
}

long int multiply(long int a, long int b) {
    /*
    Multiplies two numbers, a and b
    */
    return a * b;
}

long int divide(long int a, long int b) {
    /*
    Divides a by b
    */
    return a / b;
}

/*
My child will write safe code
*/
long int getOperand() {
    /*
    scanf
    */
    int input;
    scanf("%d", &input);
    return input;                              
}                                               