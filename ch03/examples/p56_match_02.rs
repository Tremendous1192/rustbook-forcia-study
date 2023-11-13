fn main() {
    // Create a `Result` enum with a successful variant containing the value 100
    let result: Result<i32, String> = Ok(100);

    // Print the original `Result` for debugging purposes
    println!("result: {:?}", &result);

    // Use a `match` statement to extract the value from the `Result` or handle the error
    let result_number = match result {
        // If the `Result` is Ok, extract the value
        Ok(number) => number,

        // If the `Result` is Err, handle the error by printing a message and assigning -1
        Err(message) => {
            println!("Error: {}", message);
            -1
        }
    };

    // Print the extracted value or the default value (-1) in case of an error
    println!("result_number: {:?}", result_number);
}
