// Import necessary modules from the futures crate
use futures::executor;
use futures::future::Future;

// Define a function that returns a future
fn move_to_async_block() -> impl Future<Output = ()> {
    // Create a string variable outside of the async block
    let outside_variable = "this is outside".to_string();

    // Return an async block that captures the outside variable using 'async move'
    async move {
        // Print the captured outside variable inside the async block
        println!("{}", outside_variable);
    }
}

// Main function where the program execution begins
fn main() {
    // Call the function that returns a future
    move_to_async_block();
    // Note: The future is created but not executed in this example.
    // To execute the future, you would typically use an executor like block_on.
}
