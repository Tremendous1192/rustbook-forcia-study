// Import necessary modules from the futures crate
use futures::executor;

// Define an asynchronous function that adds two integers
async fn async_add(left: i32, right: i32) -> i32 {
    left + right
}

// Define an asynchronous function that performs multiple async additions and returns the result
async fn something_great_async_function() -> i32 {
    // Call async_add three times asynchronously and await the results
    let ans1 = async_add(2, 3).await;
    let ans2 = async_add(3, 4).await;
    let ans3 = async_add(4, 5).await;

    // Combine the results using addition
    let result = ans1 + ans2 + ans3;
    result
}

// Main function where the program execution begins
fn main() {
    // Use the executor to block on the execution of the asynchronous function
    let result = executor::block_on(something_great_async_function());

    // Print the final result
    println!("{}", result);
}
