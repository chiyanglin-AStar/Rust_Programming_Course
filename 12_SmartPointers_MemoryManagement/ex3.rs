use std::rc::Rc;
use std::cell::RefCell;
// Example 3: Using RefCell<T> for mutable borrowing
fn main() {
    let value = RefCell::new(10);
    *value.borrow_mut() += 5;
    println!("Updated value: {}", value.borrow());
}
