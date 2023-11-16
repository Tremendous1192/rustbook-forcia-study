// Import necessary modules from the futures crate
use futures::executor;
use futures::future::Future;

// Define an asynchronous function that adds two integers
async fn async_add(left: i32, right: i32) -> i32 {
    left + right
}

// Define an asynchronous function that returns a Future representing the asynchronous computation
fn something_great_async_function() -> impl Future<Output = i32> {
    // Define an asynchronous block using async {}
    async {
        // Call the async_add function asynchronously and await its result
        let ans = async_add(2, 3).await;

        // Print the result
        println!("{}", ans);

        // Return the result
        ans
    }
}

// Main function where the program execution begins
fn main() {
    // Use the executor to block on the execution of the asynchronous function
    executor::block_on(something_great_async_function());
}
