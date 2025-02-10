// Example 1: Creating a module and importing it
mod greetings {
    pub fn hello() {
        println!("Hello from module!");
    }
}
fn main() {
    greetings::hello();
}
