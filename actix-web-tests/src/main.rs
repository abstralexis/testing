// I was going to try and make an async example using actix however fib is 
// literally faster sync. Memoising fib is probably a better example.

use actix_web::{web, App, HttpServer, rt::spawn};
use serde;
use sqlx;
use async_recursion::async_recursion;
use color_eyre::eyre::{Result, Report};
use thiserror::Error;
use memoise::memoise;

#[derive(Error, Debug, Clone)]
pub enum FibError {
    #[error("Argument for Fibbonacci must not be negative.")]
    NegativeArgument,
}

#[actix_web::main]
async fn main() -> Result<()> {
    //spawn(test_async_fib()).await;
    test_sync_fib();

    Ok(())
}

// There is little point in me making this use an i32 instead of a u32
// other than "people use it more" and also I wanna learn how eyre works.
#[async_recursion]
async fn nthfib(n: i32) -> Result<i32> {
    if n < 0 {
        return Err(FibError::NegativeArgument.into());
    }

    match n {
        0 | 1 => Ok(1),
        _ => Ok(spawn(nthfib(n - 1)).await?? + spawn(nthfib(n - 2)).await??)
    }
}

fn sync_nthfib(n: i32) -> Result<i32> {
    if n < 0 {
        return Err(FibError::NegativeArgument.into());
    }

    match n {
        0 | 1 => Ok(1),
        _ => Ok(sync_nthfib(n - 1)? + sync_nthfib(n - 2)?)
    }
}

async fn test_async_fib() -> Result<()> {
    for i in 1..20 {
        let result = spawn(nthfib(i)).await??;
        println!("{:?}", result);
    }

    Ok(())
}

fn test_sync_fib() -> Result<()> {
    for i in 1..20 {
        let result = sync_nthfib(i)?;
        println!("{:?}", result);
    }

    Ok(())
}