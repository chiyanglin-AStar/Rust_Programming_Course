// Example 2: Using Option<T> instead of null
fn find_element(vec: &Vec<i32>, index: usize) -> Option<i32> {
    vec.get(index).cloned()
}
fn main() {
    let numbers = vec![1, 2, 3];
    match find_element(&numbers, 1) {
        Some(value) => println!("Found: {}", value),
        None => println!("Not found"),
    }
}
