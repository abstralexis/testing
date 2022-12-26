/*
 *  This is one of my Project Euler solutions in Rust.
 *  I am going to use it as an example of functional 
 *  programming using Rust. It does not use all available
 *  functions, obviously.
 */

fn main() {
    /*
     *  Here, we are finding the difference between the 
     *  square of the sum of numbers up to n and the sum of 
     *  the square numbers up to n.
     */
    let n: i32 = 100;
    println!("{:?}", square_sum_upto(n) - sum_square_upto(n));
}

fn sum_square_upto(n: i32) -> i32 {
    /*
     *  Here, we are summing the squares of the numbers in the
     *  sequence 1 to n. The RHS is non-inclusive, so we use 
     *  n + 1.
     */
    (1..n + 1)                  // Take a sequence
        .into_iter()            // Make it iterable
        .map(|x| x.pow(2))      // Map a lambda x: x squared function to it
        .collect::<Vec<i32>>()  // Collect mapped sequence to a Vec<i32>
        .iter()                 // Take the Vec iter object
        .sum()                  // And sum from it; implicit return
}

fn square_sum_upto(n: i32) -> i32 {
    /*
     *  This takes a sequence 1 to n and squares the sum of it.
     *  This takes the sequence 1..n+1, makes it iterable, sums it 
     *  to an i32 and raises it to the power of 2. This is an implicit
     *  return.
     */
    (1..n + 1).into_iter().sum::<i32>().pow(2)
}
