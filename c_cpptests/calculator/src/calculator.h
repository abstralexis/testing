#ifndef CALC
#define CALC
#endif

// Operand enum
enum operators {addition, subtraction, division, multiplication};

// Function prototypes
long int add(long int a, long int b);

long int subtract(long int a, long int b);

long int multiply(long int a, long int b);

long int divide(long int a, long int b);

long int getOperand();

int getOperator();