fn main() {
    // Start off with some variables...
    let is_true: bool = true;
    let is_false: bool = false;
    let some_num: i32 = 32;

    // Basic if statement
    if is_true {
        println!("True");
    }

    // Use ! (a.k.a "bang") for a NOT gate
    if !is_false {
        println!("Not False");
    }

    // Use == instead of = if comparing
    if some_num == 50 {
        println!("Equal to 50");
    } else if some_num != 50 {          // Elif
        println!("Not equal to 50");
    }

    // Use an iterator with the .. operator. It is left inclusive, not right inclusive.
    for i in 0..10 {
        println!("i = {:?}", i);
    }

    println!("\n");

    // A 'manual for loop' per se with a while loop
    let mut j: u8 = 0;
    while j <= 10 {
        println!("j = {:?}", j);
        j += 1;
    }
}
