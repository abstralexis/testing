#include <stdio.h>
#include "main.h"

int main(int argc, char **argv) {
    int x = 5;                      // Here is a variable.
    int *p_x = &x;                  // This pointer points to the address (&) of x
    int deref_px = *p_x;            // Dereference the pointer to get the actual value
    printf("x: %d, p_x: %d, *p_x: %d\n", x, p_x, deref_px);
    
    /*
    This is an example of how pointers work. Here, there is no performance benefit.
    In fact, it may even be slower because you don't need a pointer to x, nor would
    it save any time. The real benefit is when instead of passing values to functions,
    you can pass them a pointer. You can do pointer arithmetic with it, and also
    it embodies a quote from someone who I forgot the name of:

    "It's easier to give your friends your address rather than move your whole house to them"
    or something like that.
    */

    int y = 10;
    int *p_y = &y;
    int result = f(p_x, p_y);
    printf("%d", result);

    return 0;
}

int f(int* arg1, int* arg2) {
    // Do something complex
    return *arg1 + *arg2;
}