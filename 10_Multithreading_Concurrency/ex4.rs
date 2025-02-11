use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
//extern crate tokio;
//use std::thread;
//use std::time::Duration;
//use tokio::time;
// Example 4: Sending messages between threads using channels
fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send("Message from thread").unwrap();
    });
    println!("Received: {}", rx.recv().unwrap());
}
