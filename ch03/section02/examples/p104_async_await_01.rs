// Import the necessary modules from the futures crate
use futures::executor;

// Define an asynchronous function that adds two integers
async fn async_add(left: i32, right: i32) -> i32 {
    left + right
}

// Define another asynchronous function that uses the previous async function
async fn something_great_async_function() -> i32 {
    // Call the async_add function asynchronously and await its result
    let ans = async_add(2, 3).await;

    // Print the result
    println!("{}", ans);

    // Return the result
    ans
}

// Main function where the program execution begins
fn main() {
    // Use the executor to block on the execution of the asynchronous function
    executor::block_on(something_great_async_function());
}
