// Example 3: Using tuple structs
struct Point(i32, i32);
fn main() {
    let p = Point(3, 4);
    println!("Point coordinates: ({}, {})", p.0, p.1);
}
