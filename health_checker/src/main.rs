
// #[tokio::main] - This is a procedural macro [an attribute macro] It intercepts the main function before it compiles and injects all the boilerplate code needed to start a multithreaded Green Thread Pool [The tokio runtime]
// Without this we cannot use the .await keyword
#[tokio::main]
// async fn main() - We added the async keyword to main function. This allows us to use .await inside the function body. The macro above makes this valid
async fn main() {
    println!("Tokio runtime has started~");
}
