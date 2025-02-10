//extern crate my_project;
mod my_project1;
//pub use my_project1::helper;
pub fn helper() {
    println!("Helper function");
}
//mod my_project1;
use my_project1::helper;
fn main() {
    helper();
}
