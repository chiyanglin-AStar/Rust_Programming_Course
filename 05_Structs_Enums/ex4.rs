// Example 4: Defining enums for multiple states
enum Status {
    Active,
    Inactive,
    Pending,
}
fn main() {
    let status = Status::Inactive;
    match status {
        Status::Active => println!("Status: Active"),
        Status::Inactive => println!("Status: Inactive"),
        Status::Pending => println!("Status: Pending"),
    }
}
