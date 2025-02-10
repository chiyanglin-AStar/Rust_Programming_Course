// Example 5: Exploring lifetimes ('a annotations)
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
fn main() {
    let string1 = String::from("Rust");
    let string2 = "Programming";
    let result = longest(&string1, string2);
    println!("Longest string: {}", result);
}
