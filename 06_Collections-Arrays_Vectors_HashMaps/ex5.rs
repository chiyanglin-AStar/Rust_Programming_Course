// Example 5: Using HashMap<K, V> for key-value storage
use std::collections::HashMap;
fn main() {
    let mut map = HashMap::new();
    map.insert("Alice", 25);
    map.insert("Bob", 30);
    println!("Alice's age: {:?}", map.get("Alice"));
}
