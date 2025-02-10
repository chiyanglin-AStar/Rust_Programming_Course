// Example 5: Using loop with break and continue
fn main() {
    let mut num = 0;
    loop {
        num += 1;
        if num == 3 {
            continue; // Skips printing 3
        }
        if num > 5 {
            break; // Exits loop when num > 5
        }
        println!("Num: {}", num);
    }
}
