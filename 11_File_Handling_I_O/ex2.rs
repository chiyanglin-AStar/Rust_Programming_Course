use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};
use serde_json::Value;
// Example 2: Writing to a file
fn main() {
    let mut file = File::create("output.txt").expect("Failed to create file");
    file.write_all(b"Hello, Rust!").expect("Failed to write to file");
}
