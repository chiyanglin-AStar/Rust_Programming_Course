// Example 5: Writing an async function
use tokio;
use tokio::time;
use std::time::Duration;
#[tokio::main]
async fn main() {
    async fn say_hello() {
        time::sleep(Duration::from_secs(1)).await;
        println!("Hello from async function!");
    }
    say_hello().await;
}