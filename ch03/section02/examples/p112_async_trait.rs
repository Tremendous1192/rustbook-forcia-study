// Import the `async_trait` crate to use the #[async_trait] attribute
use async_trait::async_trait;

// Define a trait named 'AsyncTrait' with an asynchronous function 'f'
#[async_trait]
pub trait AsyncTrait {
    async fn f(&self);
}

// Define a struct named 'Runner'
struct Runner {}

// Implement the 'AsyncTrait' trait for the 'Runner' struct
#[async_trait]
impl AsyncTrait for Runner {
    // Implement the asynchronous function 'f' for 'Runner'
    async fn f(&self) {
        println!("Hello, async-trait");
    }
}

// Define the main function
fn main() {
    // Create an instance of 'Runner'
    let runner = Runner {};
    // Call the asynchronous function 'f' on 'runner'
    let _ = runner.f();
}
