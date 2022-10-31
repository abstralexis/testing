// Don't use this normally. This is just to avoid the yellow dots and lines.
#![allow(unused_mut)]
#![allow(unused_variables)]

/*
This is to explore how functions work in Rust with some basic
examples
*/

fn main() {
    println!("Hello, world!");
    some_function();                        // Call a function

    let some_str = String::from("Foo");     // Make a variable
    fn_with_args(some_str);                 // Pass the variable in!

    let num: i32 = 10;                      // Make a variable
    let doubled: i32 = fn_with_return(num); // Assign output to new variable!
    println!("{}", doubled);                // Print the output!

    let mut bar: i32 = 16;                  // Here is a mutable variable!
    let mut baz: i32 = 25;                  // And another!
    borrower(&bar, &mut baz);               // Borrow them with "&"!
    println!("bar: {} baz: {}", bar, baz);  // Print the values
}

fn some_function() {
    println!("This is some_function!");     // This is a function!
}

fn fn_with_args(s: String) {                // Take in an argument
    println!("{}", s);                      // Print the argument!
}

fn fn_with_return(x: i32) -> i32 {          // This function returns an i32!
    let two_times_x: i32 = 2 * x;           // Double the value
    return two_times_x;                     // Return the value!
}

fn borrower(y: &i32, z: &mut i32) {         // Borrow the values!
    // y = 42;                              // Fails, is read-only
    *z = 29;                                // Dereference z to assign!
}