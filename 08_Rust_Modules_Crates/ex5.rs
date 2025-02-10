mod library {
    pub struct Book {
        pub title: String,
        author: String,
    }
    impl Book {
        pub fn new(title: &str, author: &str) -> Book {
            Book {
                title: title.to_string(),
                author: author.to_string(),
            }
        }
    }
}
fn main() {
    let book = library::Book::new("Rust Programming", "John Doe");
    println!("Book Title: {}", book.title);
}

