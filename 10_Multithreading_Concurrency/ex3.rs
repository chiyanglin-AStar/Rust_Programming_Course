use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
//extern crate tokio;
//use std::thread;
//use std::sync::mpsc;
//use std::time::Duration;
//use tokio::time;
// Example 3: Sharing ownership with Arc<T>
fn main() {
    let data = Arc::new(vec![1, 2, 3]);
    let mut handles = vec![];
    for _ in 0..3 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("Thread sees: {:?}", data);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
