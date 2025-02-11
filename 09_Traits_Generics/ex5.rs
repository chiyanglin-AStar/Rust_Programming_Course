// Example 5: Deriving built-in traits like Debug and Clone
#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}
fn main() {
    let p1 = Point { x: 10, y: 20 };
    let p2 = p1.clone();
    println!("Original: {:?}, Cloned: {:?}", p1, p2);
}
