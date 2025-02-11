use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
//extern crate tokio;
//use std::thread;
//use std::sync::mpsc;
//use std::time::Duration;
//use tokio::time;
// Example 2: Using Mutex<T> for shared state
fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Final counter value: {}", *counter.lock().unwrap());
}
