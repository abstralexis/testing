#ifndef MAIN_H
#define MAIN_H

// Here, we use a header file because the compiler doesn't know
// what f() is and needs to see a prototype. It is often easier to 
// put these prototypes into a header (.h/.hh).
int f(int* arg1, int* arg2);

#endif