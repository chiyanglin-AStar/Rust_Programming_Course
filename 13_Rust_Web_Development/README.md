# Rust_Programming_Course

# **📚 Rust Programming Course Outline**

---

## **1️⃣3️⃣ Rust for Web Development**
- Using Rocket or Actix Web
- Building a REST API in Rust
- Handling HTTP Requests
- JSON Serialization & Deserialization

### **Examples**
1. Setting up a Rocket web server
2. Creating an API endpoint
3. Handling GET and POST requests
4. Parsing JSON with `serde`
5. Returning structured API responses


[30 天用 Rust 打造 QR Code 製造機 ](https://ithelp.ithome.com.tw/users/20120293/ironman/6217)


// 13. Rust for Web Development
use rocket::{get, post, routes, serde::json::Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Message {
    content: String,
}

// Example 1: Setting up a Rocket web server
#[get("/")]
fn index() -> &'static str {
    "Hello, Rocket!"
}

// Example 2: Creating an API endpoint
#[post("/message", format = "json", data = "<msg>")]
fn create_message(msg: Json<Message>) -> Json<Message> {
    Json(Message {
        content: format!("Received: {}", msg.content),
    })
}

// Example 3: Handling GET and POST requests
#[rocket::main]
async fn main() {
    let _ = rocket::build().mount("/", routes![index, create_message]).launch().await;
}

// Example 4: Parsing JSON with serde
fn parse_json() {
    let data = r#"{"content": "Hello, World!"}"#;
    let parsed: Message = serde_json::from_str(data).unwrap();
    println!("Parsed message: {}", parsed.content);
}

// Example 5: Returning structured API responses
fn structured_response() -> Json<Message> {
    Json(Message {
        content: "This is a structured response".to_string(),
    })
}
