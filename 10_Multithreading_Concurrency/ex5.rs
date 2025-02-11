use std::thread;
use std::time::Duration;
extern crate tokio;
//use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::rc::Rc;
use tokio::task::JoinHandle;
//use std::time::Duration;
//use tokio::time;
// Example 5: Writing an async function
#[tokio::main]
async fn main() {
    async fn say_hello() {
        time::sleep(Duration::from_secs(1)).await;
        println!("Hello from async function!");
    }
    say_hello().await;
}
#[tokio::main]
async fn main() {
    async fn say_hello() {
        time::sleep(Duration::from_secs(1)).await;
        println!("Hello from async function!");
    }
    say_hello().await;
}