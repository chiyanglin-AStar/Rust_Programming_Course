use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
//extern crate tokio;
//use std::thread;
//use std::sync::mpsc;
//use std::time::Duration;
//use tokio::time;




// Example 1: Spawning threads with thread::spawn
fn main() {
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Spawned thread: {}", i);
            thread::sleep(Duration::from_millis(500));
        }
    });
    handle.join().unwrap();
}
