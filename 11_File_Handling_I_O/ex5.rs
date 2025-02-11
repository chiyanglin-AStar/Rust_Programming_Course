extern crate serde_json;
use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};
use serde_json::Value;
// Example 5: Parsing JSON using serde_json
fn main() {
    let json_data = r#"{"name": "Alice", "age": 30}"#;
    let parsed: Value = serde_json::from_str(json_data).expect("Failed to parse JSON");
    println!("Parsed JSON: {}", parsed["name"]);
}
