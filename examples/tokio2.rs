use std::thread;

use anyhow::Result;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel(32);
    let handler = worker(rx);

    tokio::spawn(async move {
        let mut i = 0;
        loop {
            println!("Send {i}");
            i += 1;
            tx.send("Future 1".to_string()).await?;
        }
        #[allow(unreachable_code)]
        Ok::<(), anyhow::Error>(())
    });

    handler.join().unwrap();

    Ok(())
}

fn worker(mut rx: mpsc::Receiver<String>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        while let Some(s) = rx.blocking_recv() {
            let ret = expensive_blocking_task(s);
            println!("result: {ret}");
        }
    })
}

fn expensive_blocking_task(s: String) -> String {
    thread::sleep(std::time::Duration::from_millis(300));
    blake3::hash(s.as_bytes()).to_string()
}
