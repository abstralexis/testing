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

    printf("\nInput an operator number (1-4, +-*/)");
    int operator = getOperator();
    printf("The operand is: %d\n", operator);

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
}   // Lots of undefined behaviour. Fun.

/*
Please child, write safe code-
*/
int getOperator() {
    /*
    SCANF
    */
    int input;
    scanf("%d", &input);
    if (input > 0 && input <= 4) {  // If input 1-4 incl.
        return input;
    } else {
        return -1;                  // -1 on invalid
    }
}   // Uses the undefined behaviour in a good way (maybe)