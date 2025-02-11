// Example 2: Implementing a trait for a struct
trait Area {
    fn area(&self) -> f64;
}
struct Circle {
    radius: f64,
}
impl Area for Circle {
    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}
fn main() {
    let circle = Circle { radius: 5.0 };
    println!("Circle Area: {}", circle.area());
}
