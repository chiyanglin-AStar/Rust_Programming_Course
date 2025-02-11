// Example 1: Creating a simple trait
trait Greet {
    fn greet(&self) -> String;
}
struct Person {
    name: String,
}
impl Greet for Person {
    fn greet(&self) -> String {
        format!("Hello, {}!", self.name)
    }
}
fn main() {
    let person = Person { name: "Alice".to_string() };
    println!("{}", person.greet());
}
