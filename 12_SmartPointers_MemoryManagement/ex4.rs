use std::rc::Rc;
use std::cell::RefCell;
// Example 4: Combining Rc<T> and RefCell<T>
struct Data {
    value: RefCell<i32>,
}
fn main() {
    let shared_data = Rc::new(Data { value: RefCell::new(42) });
    let shared_clone = Rc::clone(&shared_data);
    *shared_clone.value.borrow_mut() += 8;
    println!("Updated shared value: {}", shared_data.value.borrow());
}
