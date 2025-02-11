# Rust_Programming_Course

# **📚 Rust Programming Course Outline**
Each topic contains **over 5 practical examples** to reinforce learning.

---

## **🔟 Multithreading & Concurrency**
- Threads & `std::thread`
- Shared State with `Mutex` & `Arc`
- Channels for Message Passing
- `async` & `await` in Rust

### **Examples**
1. Spawning threads with `thread::spawn`
2. Using `Mutex<T>` for shared state
3. Sharing ownership with `Arc<T>`
4. Sending messages between threads using channels
5. Writing an `async` function

```rustc
rustc --edition=2018 --crate-type=lib xxx.rs

```
[async ref:Rust Async IO: A Beginner’s Guide to Asynchronous Programming in Rust](https://medium.com/@tzutoo/rust-async-io-a-beginners-guide-to-asynchronous-programming-in-rust-600219226c82)


