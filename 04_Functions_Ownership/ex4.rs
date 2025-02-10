// Example 4: Using borrowing & references (&)
fn print_length(s: &String) {
    println!("Length: {}", s.len());
}
fn main() {
    let s = String::from("Hello");
    print_length(&s);
    println!("Still own s: {}", s);
}
