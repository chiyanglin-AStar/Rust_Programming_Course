use tokio;

async fn operation1() {
    // some async work
    println!("async op1");
}

async fn operation2() {
    // some async work
    println!("async op2");
}

#[tokio::main]
async fn main() {
    let (res1, res2) = tokio::join!(operation1(), operation2());
}
