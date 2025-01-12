use std::{
    thread::{self},
    time::Duration,
};

use tokio::{fs, runtime::Builder, time::sleep};

fn main() {
    let handle = thread::spawn(|| {
        // execute future
        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        rt.spawn(async {
            println!("Future 1");
            let content = fs::read_to_string("Cargo.toml").await.unwrap();
            println!("Content Length: {}", content.len());
        });

        rt.spawn(async {
            println!("Future 2");
            let rt = expensive_blocking_task("Future 2".to_string());
            println!("result: {rt}");
        });

        // rt.block_on(async {
        //     println!("Hello World");
        // })
        rt.block_on(async {
            sleep(Duration::from_millis(900)).await;
        });
    });
    handle.join().unwrap();
}

fn expensive_blocking_task(s: String) -> String {
    thread::sleep(std::time::Duration::from_millis(300));
    blake3::hash(s.as_bytes()).to_string()
}
