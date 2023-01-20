use rand::prelude::*;

fn main() {
    println!("{:?}", gen_ascii_chars(64));
}

fn gen_ascii_chars(length: u32) -> String {
    let arr: Vec<u32> = (0..length)
        .collect::<Vec<u32>>()
        .into_iter()
        .map(|_| thread_rng().gen())
        .collect::<Vec<u32>>();
    arr
        .into_iter()
        .map(|x| x.to_string())
        .collect::<String>()
}