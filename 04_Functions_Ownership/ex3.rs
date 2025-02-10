// Example 3: Understanding ownership in Rust
fn main() {
    let s = String::from("Hello");
    takes_ownership(s);
    // println!("{}", s); // This would cause an error
}
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
