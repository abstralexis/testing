/*
This is my attempt at making a basic calculator in C
*/

#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

#include "calculator.h"

int main(int argc, char **argv) {
    /*
    Main function

    No clue why it all uses long ints, maybe because
    of some code I tried off of SO to try and get safe
    scanf int input
    */

    long int operandOne = getOperand();
    long int operandTwo = getOperand();

    int operator;
    bool isValidOperator = false;
    /* 
    For some reason this can cause an infinite loop
    If the input is invalid. That's C undefined
    behaviour for ya.
    */ 
    while (!isValidOperator) {
        operator = getOperator();
        if (operator != -1) {
            isValidOperator = true;
        } else {
            printf("\nInvalid operator. Try again.\n");
        }
    }

    long int result = calculate(operandOne, operandTwo, operator);
    if (&result != NULL) { printf("Result = %d\n", result); }
    else { printf("Result was null"); }

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
    printf("Please input an operand.\n");
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
    printf("Please input an operator. 1:+ 2:- 3:* 4:/\n");
    int input;
    scanf("%d", &input);
    if (input > 0 && input <= 4) {  // If input 1-4 incl.
        return input;
    } else {
        return -1;                  // -1 on invalid
    }
}   // Uses the undefined behaviour in a good way (maybe)

/*
For the last time child, write safe code-
*/
long int calculate(long int a, long int b, int operator) {
    /*
    Infers that operator is a valid operator >:)
    else returns NULL
    */
    if (operator == 1) { return (a + b); }
    else if (operator == 2) { return (a - b); }
    else if (operator == 3) { return (a * b); }
    else if (operator == 4) { return (a / b); }
    else { return NULL; }
}