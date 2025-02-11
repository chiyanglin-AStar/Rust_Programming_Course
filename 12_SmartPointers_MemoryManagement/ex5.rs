use std::rc::Rc;
use std::cell::RefCell;
// Example 5: Implementing custom smart pointers
use std::ops::Deref;
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}
fn main() {
    let custom_box = MyBox::new(100);
    println!("Custom Box value: {}", *custom_box);
}

