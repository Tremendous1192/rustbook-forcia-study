// Define an asynchronous function named 'add' that takes two integers and returns their sum
async fn add(left: i32, right: i32) -> i32 {
    left + right
}

// Define the asynchronous main function, the entry point for the program
#[async_std::main]
async fn main() {
    // Call the 'add' asynchronous function with arguments 2 and 3, await the result, and store it in 'ans'
    let ans = add(2, 3).await;
    // Print the result of the addition
    println!("{}", ans);
}
