# Rust_Programming_Course

# **📚 Rust Programming Course Outline**
Each topic contains **over 5 practical examples** to reinforce learning.

---
## **4️⃣ Functions & Ownership**
- Defining Functions
- Function Parameters & Return Values
- Ownership, Borrowing, & Lifetimes
- Passing Variables by Reference

### **Examples**
1. Creating functions with parameters
2. Returning values from functions
3. Understanding ownership in Rust
4. Using borrowing & references (`&`)
5. Exploring lifetimes (`'a` annotations)
   
[What Is Ownership?](https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html)

[什麼是所有權？](https://rust-lang.tw/book-tw/ch04-01-what-is-ownership.html#%E4%BB%80%E9%BA%BC%E6%98%AF%E6%89%80%E6%9C%89%E6%AC%8A)


6. **Ownership Transfer**:

   ```rust
        fn main() {
            let s1 = String::from("hello");
            let s2 = s1;
            // println!("{}", s1); // This would cause a compile-time error
            println!("{}", s2);
        }
   ```
  7. **Borrowing**:
        ```rust
        fn main() {
            let s1 = String::from("hello");
            let len = calculate_length(&s1);
            println!("The length of '{}' is {}.", s1, len);
        }

        fn calculate_length(s: &String) -> usize {
            s.len()
        }
        ```
  8. **Mutable Borrow**:
        ```rust
        fn main() {
            let mut s = String::from("hello");
            change(&mut s);
            println!("{}", s);
        }

        fn change(s: &mut String) {
            s.push_str(", world");
        }
        ```
  9. **Lifetimes**:
        ```rust
        fn main() {
            let string1 = String::from("long string is long");
            let result;
            {
                let string2 = String::from("xyz");
                result = longest(string1.as_str(), string2.as_str());
            }
            println!("The longest string is {}", result);
        }

        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
        ```
   10. **Dangling Reference Prevention**:
        ```rust
        fn main() {
            let reference_to_nothing = dangle();
        }

        fn dangle() -> &String {
            let s = String::from("hello");
            &s // Compile-time error: `s` does not live long enough
        }
        ```



