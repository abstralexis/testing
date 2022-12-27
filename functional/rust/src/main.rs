fn main() {
    println!(
        "{:?}", 
        (1..101)                    // Take a range
            .into_iter()            // Make the range iterable
            .filter(|x| x % 2 == 0) // Filter with anonymous even filter
            .collect::<Vec<i32>>()  // Collect to a Vec of i32
    );
}
