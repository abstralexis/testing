/*
This file aims to show how the Box smart pointer can
be used in Rust. 
They are used when you do not know the memory size of
something at compile time to add to the stack, so they 
must be dynamically allocated to the heap.
*/

// A recursive linked list, otherwise known as a Cons List.
#[derive(Debug)]            // This allows us to print, for example.
enum List<T> {              // T is what's known as a type generic. It is used
    Cons(T, Box<List<T>>),  // to denote that any type can be used.
    Nil,                    // A box is needed as it has a fixed size.
}                           // This means we can compile the code.

use List::*;                // So we don't have to do List::Cons etc

fn main() {
    // You can make a basic Box smart pointer as follows:
    let some_box: Box<i32> = Box::new(5);   // This example is fairly useless.
    
    // How we use the Cons List
    let some_list: List<i32> = // Not the prettiest but it is what it is.
        Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("{}, {:?}", some_box, some_list);
}