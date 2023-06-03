use memoize::memoize;
use std::time::Instant;

fn main() {
    let instant = Instant::now();
    test_fib();
    println!("Unmemoised: {:?}", instant.elapsed());

    let instant = Instant::now();
    test_mfib();
    println!("Memoised: {:?}", instant.elapsed());
}

#[memoize]
fn mfib(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        _ => mfib(n - 1) + mfib(n - 2)
    }
}

fn fib(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        _ => fib(n - 1) + fib(n - 2)
    }
}

fn test_fib() {
    for i in 0..30 { fib(i); }
}

fn test_mfib() {
    for i in 0..30 { mfib(i); }
}