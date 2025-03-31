// Example 5: Shadowing variables
fn main() {
    let x = 5;
    let x = x + 1; // Shadowing x
    {
        let x = x * 2;
        println!("Inner x: {}", x);
    }
    println!("Outer x: {}", x);
}
