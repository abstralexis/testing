fn main() {
    println!(
        "{:?}",
        (1..101)
            .into_iter()
            .filter(|x| x % 2 == 0)
            .collect::<Vec<i32>>()
    );
}
