extern crate serde_json;
use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};
use serde_json::Value;

// Example 1: Reading a file with std::fs
fn main() {
    let content = fs::read_to_string("example.txt").expect("Failed to read file");
    println!("File content: {}", content);
}
