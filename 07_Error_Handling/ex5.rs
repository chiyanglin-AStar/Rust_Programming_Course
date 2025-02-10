// Example 5: Custom error types with enum
#[derive(Debug)]
enum MyError {
    NotFound,
    PermissionDenied,
}
fn check_access(user: &str) -> Result<(), MyError> {
    match user {
        "admin" => Ok(()),
        _ => Err(MyError::PermissionDenied),
    }
}
fn main() {
    match check_access("guest") {
        Ok(_) => println!("Access granted"),
        Err(e) => println!("Access denied: {:?}", e),
    }
}
