// Example 2: Implementing methods with impl
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn greet(&self) {
        println!("Hello, my name is {}, my age is {}!", self.name,self.age);
    }
}
fn main() {
    let person = Person { name: String::from("Alice"), age: 30 };
    person.greet();
}
