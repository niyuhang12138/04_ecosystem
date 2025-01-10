use anyhow::Context;
use std::{fs, io};

use std::collections::HashMap;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),

    #[error("Serialize json error: {0}")]
    Serialize(#[from] serde_json::Error),

    #[error("Big error: {0:?}")]
    BigError(Box<BigError>),

    #[error("Custom error: {0}")]
    Custom(String),
}

#[allow(unused)]
#[derive(Debug)]
pub struct BigError {
    a: String,
    b: Vec<String>,
    c: HashMap<String, String>,
}

fn main() -> Result<(), anyhow::Error> {
    println!("size of anyhow::Error is {}", size_of::<anyhow::Error>());
    println!("size of io::Error is {}", size_of::<io::Error>());
    println!(
        "size of serialize::Error is {}",
        size_of::<serde_json::Error>()
    );
    println!("size of string::Error is {}", size_of::<String>());
    println!("size of MyError is {}", std::mem::size_of::<MyError>());

    let filename = "not-existent-file.txt";
    let _file = fs::File::open(filename).with_context(|| format!("这个文件: {filename}找不到"))?;

    fail_with_error()?;

    Ok(())
}

fn fail_with_error() -> Result<(), MyError> {
    Err(MyError::Custom("This is a custom error".to_string()))
}
