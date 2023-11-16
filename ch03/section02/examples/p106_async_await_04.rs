// Import necessary modules from the futures crate
use futures::executor;

// Define an asynchronous function that adds two integers
async fn async_add(left: i32, right: i32) -> i32 {
    left + right
}

// Define an asynchronous function that prints a given value
async fn print_result(value: i32) {
    println!("{}", value);
}

// Define an asynchronous function that performs a series of async additions and prints the results
async fn calculate() -> i32 {
    // Perform the first async addition
    let add1 = async_add(2, 3).await;
    // Print the result using the print_result function
    print_result(add1);

    // Perform the second async addition
    let add2 = async_add(3, 4).await;
    // Print the result using the print_result function
    print_result(add2);

    // Perform a final async addition using the results of the previous additions
    async_add(add1, add2).await
}

// Main function where the program execution begins
fn main() {
    // Use the executor to block on the execution of the asynchronous function
    let result = executor::block_on(calculate());

    // Print the final result
    println!("{}", result);
}
