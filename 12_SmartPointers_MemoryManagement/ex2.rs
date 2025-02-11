use std::rc::Rc;
use std::cell::RefCell;
// Example 2: Sharing ownership with Rc<T>
fn main() {
    let shared = Rc::new(5);
    let shared_clone = Rc::clone(&shared);
    println!("Shared value: {}", shared_clone);
    println!("Reference count: {}", Rc::strong_count(&shared));
}
