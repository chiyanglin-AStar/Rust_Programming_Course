use std::rc::Rc;
use std::cell::RefCell;

// Example 1: Using Box<T> for dynamic memory allocation
fn main() {
    let boxed_value = Box::new(10);
    println!("Boxed value: {}", boxed_value);
}
