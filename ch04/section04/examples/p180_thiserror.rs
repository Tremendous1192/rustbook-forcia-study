use thiserror::Error;

// Define an error enum with custom error messages and transparently wrapping ParseIntError
#[derive(Error, Debug)]
enum MyError {
    #[error("failed to read string from {0}")]
    ReadError(String), // Variant for I/O errors with a custom error message
    #[error(transparent)]
    ParseError(#[from] std::num::ParseIntError), // Variant transparently wrapping ParseIntError
}

// Function to read an integer from a file, returning a Result with MyError as the error type
fn get_int_from_file() -> Result<i32, MyError> {
    // Specify the file path
    let path = "number_p178.txt";

    // Read the file content into a string, converting I/O errors to MyError::ReadError variant
    let num_str = std::fs::read_to_string(path).map_err(|e| MyError::ReadError(path.into()))?;

    // Parse the string as an integer, multiplying the result by 2, or convert parsing errors to MyError::ParseError variant
    num_str
        .trim()
        .parse::<i32>()
        .map(|t| t * 2)
        .map_err(MyError::from)
}

// Main function to demonstrate error handling
fn main() {
    // Match on the Result returned by get_int_from_file
    match get_int_from_file() {
        // If successful, print the result
        Ok(x) => println!("{}", x),
        // If an error occurred, print the detailed error information using {:#?}
        Err(e) => println!("{:#?}", e),
    }
}
