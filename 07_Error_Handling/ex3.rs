// Example 3: Using unwrap() vs expect()
fn main() {
    let value = Some(42);
    println!("Unwrapped value: {}", value.unwrap());
    println!("Expected value: {}", value.expect("Value should be present"));
}
