// Turn off warning for demonstration purposes
#![allow(unused_assignments)]

fn main() {
    /*
    This is for demonstrating syntax for declaring variables
    and using them in basic ways.
    */
    let immutable_number: i32 = 0;      // Rust variables are immutable by default.
    // immutable_number = 1;            // This would not compile.
    let mut mutable_number: i32 = 5;    // Set a mutable variable with "mut"
    mutable_number = 10;                // Change the mutable variable
    println!("immutable: {} mutable: {}", immutable_number, mutable_number);
}
