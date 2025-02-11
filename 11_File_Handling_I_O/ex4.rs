use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};
use serde_json::Value;
// Example 4: Reading user input from stdin
fn main() {
    let mut input = String::new();
    println!("Enter your name:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    println!("Hello, {}", input.trim());
}
