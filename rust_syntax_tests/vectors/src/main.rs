/*
Demonstrating how vectors work in Rust.
They are Rust's implementation of arrays.
*/

fn main() {
    // Creating a vector with Vec::new()
    let mut vector: Vec<i32> = Vec::new();
    vector.push(1);             // Make a new vector and push values to it
    vector.push(2);
    //vector.push("Hello");     // This fails because vecs only store one type
    println!("{:?}", vector);   // Prints [1, 2]
    vector.pop();               // Remove last item in vec
    println!("{:?}", vector);   // Prints [1]

    // Creating a vector with the vec! macro
    let macro_vec: Vec<&str> = vec!("Hello", "World");
    println!("{:?}", macro_vec);    // Prints ["Hello", "World"]

    // Other ways to create a vec quickly:
    // Create it with a set starting capacity
    let mut capacity_vec: Vec<i32> = Vec::with_capacity(5);
    capacity_vec.resize(5, 0);  // Fill the vec with 5 zeroes
    // Or do the same quicker with a macro:
    let macro_capacity_vec: Vec<i32> = vec![0; 5];
    // Prints [0, 0, 0, 0, 0] [0, 0, 0, 0, 0]
    println!("{:?} {:?}", capacity_vec, macro_capacity_vec);

    // You can also derive a vec from an iterator
    let collected: Vec<i32> = (0..10).collect();
    println!("{:?}", collected);    // Prints [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

    // You can also iterate through them easily like in Python:
    for i in collected {
        println!("{:?}", i);    // Prints 1 to 10 on new lines.
    }

    // You can even make 2D vectors!
    let two_dimensional: Vec<Vec<i32>> = vec![vec![0; 5]; 5];   // 5x5 of 0s
    println!("{:?}", two_dimensional);  // Print the array

    /*
    As you can see, using the vec! macro can be very useful and
    a very good time saver. But this is not just for writing -
    it can also save some compilation and run time too!!
    */
}
