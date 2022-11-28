/*
This is my attempt at making a basic calculator in C
*/

#include <stdio.h>

#include "calculator.h"

int main(int argc, char **argv) {
    printf("Hello World!\n");

    int a = 10;
    int b = 100;
    int o = add(a, b);

    printf("The output is: %d\n", o);

    return 0;
}

int add(int a, int b) {
    return a + b;
}