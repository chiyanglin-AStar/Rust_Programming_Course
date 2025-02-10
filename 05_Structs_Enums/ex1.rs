// Example 1: Creating & using a struct
struct Person {
    name: String,
    age: u8,
}
fn main() {
    let person = Person { name: String::from("Alice"), age: 30 };
    println!("{} is {} years old.", person.name, person.age);
}
