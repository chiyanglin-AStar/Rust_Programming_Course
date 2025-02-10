// Example 2: Using mod and use
mod math {
    pub mod operations {
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }
    }
}
use math::operations::add;
fn main() {
    println!("Sum: {}", add(5, 10));
}
