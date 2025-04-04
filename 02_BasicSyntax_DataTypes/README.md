# Rust_Programming_Course

# **📚 Rust Programming Course Outline**
Each topic contains **over 5 practical examples** to reinforce learning.

---

## **2️⃣ Basic Syntax & Data Types**
- Variables & Mutability
- Data Types (`i32`, `f64`, `bool`, `char`, `&str`)
- Constants & Shadowing
- Type Annotations

### **Examples**
1. Declaring variables with `let` and `mut`
2. Using different number types (integer, floating-point)
3. Exploring `bool` and `char`
4. Understanding `const` vs `let`
5. Shadowing variables

6. **Integer Types**:
```rust
 fn main() {
            let a: i32 = 10;
            let b: u32 = 20;
            println!("a = {}, b = {}", a, b);
        }       
```
7. **Floating Point Types**:
```rust
fn main() {
            let x: f64 = 3.1412345678901234567890123456789;
            let y: f32 = 2.7112345678901234567890123456789;
            println!("x = {}, y = {}", x, y);
        }
```
```rust
//https://stackoverflow.com/questions/26576889/how-do-i-print-a-rust-floating-point-number-with-all-available-precision

fn main() {
            let x: f64 = 3.1412345678901234567890123456789;
            let y: f32 = 2.7112345678901234567890123456789;
            println!("x = {:.32}, y = {:.12}", x, y);
        }
```
