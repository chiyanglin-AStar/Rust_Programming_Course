# Rust_Programming_Course

# **📚 Rust Programming Course Outline**
Each topic contains **over 5 practical examples** to reinforce learning.

---

## **1️⃣ Introduction to Rust**
- What is Rust?
- Installing Rust (Rustup, Cargo)
- Writing & Running Your First Rust Program
- Rust Toolchain (Cargo, Crates, Rustfmt, Clippy)

[ref:](https://www.tutorialspoint.com/rust/rust_helloworld_example.htm)

### **Examples**
1. Installing Rust using `rustup`
  ```rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

rustc --version
  ```
2. Writing a "Hello, World!" program
```rust
  fn main(){
     println!("Rust says Hello to TutorialsPoint !!");
  }
  
  rustc hello.rs 
  ./hello

```
3. Using Cargo to build & run a project
```rust
cargo new hello
cd hello
tree .
.
├── Cargo.toml
└── src
    └── main.rs

1 directory, 2 files
cat Cargo.toml
[package]
name = "helloworld"
version = "0.1.0"
edition = "2021"

[dependencies]
cat ./src/main.rs
fn main() {
    println!("Hello, world!");
}
cargo build
cargp run 

```
4. Exploring Cargo.toml & dependencies
5. Using `rustc` to compile Rust code

