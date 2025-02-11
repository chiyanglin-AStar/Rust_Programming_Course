// Example 3: Using generics in a function
fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
fn main() {
    println!("Sum: {}", add(10, 20));
    println!("Sum: {}", add(1.5, 2.5));
}
