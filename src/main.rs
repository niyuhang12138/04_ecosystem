use anyhow::Context;
use ecosystem::MyError;
use std::{fs, io};

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
