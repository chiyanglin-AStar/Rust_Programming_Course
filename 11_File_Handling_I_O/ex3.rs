use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};
use serde_json::Value;
// Example 3: Handling file reading errors
fn main() {
    match fs::read_to_string("nonexistent.txt") {
        Ok(content) => println!("File content: {}", content),
        Err(e) => println!("Error reading file: {}", e),
    }
}
