// Example 4: Implementing multiple traits on a type
trait Shape {
    fn draw(&self);
}
trait Resize {
    fn resize(&mut self, factor: f64);
}
struct Square {
    size: f64,
}
impl Shape for Square {
    fn draw(&self) {
        println!("Drawing a square of size {}", self.size);
    }
}
impl Resize for Square {
    fn resize(&mut self, factor: f64) {
        self.size *= factor;
    }
}
fn main() {
    let mut square = Square { size: 10.0 };
    square.draw();
    square.resize(1.5);
    square.draw();
}
