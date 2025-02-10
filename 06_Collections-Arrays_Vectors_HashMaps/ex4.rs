// Example 4: Iterating over a Vec<T>
fn main() {
    let vec = vec![100, 200, 300];
    for value in &vec {
        println!("Value: {}", value);
    }
}
