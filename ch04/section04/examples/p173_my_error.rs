enum MyError {
    Io(std::io::Error),           // Variant representing I/O errors
    Num(std::num::ParseIntError), // Variant representing parsing errors for integers
}

fn get_int_from_file() -> Result<i32, MyError> {
    // Specify the file path
    let path = "number.txt";

    // Read the file content into a string, converting I/O errors to MyError::Io variant
    let num_str = std::fs::read_to_string(path).map_err(|e| MyError::Io(e))?;

    // Parse the string as an integer, multiplying the result by 2, or convert parsing errors to MyError::Num variant
    num_str
        .trim()
        .parse::<i32>()
        .map(|t| t * 2)
        .map_err(|e| MyError::Num(e))
}

fn main() {
    // Match on the Result returned by get_int_from_file
    match get_int_from_file() {
        // If successful, print the result
        Ok(x) => println!("{}", x),
        // If an error occurred, match on the specific variant of MyError and print the cause
        Err(e) => match e {
            MyError::Io(cause) => println!("I/O Error: {}", cause),
            MyError::Num(cause) => println!("Parse Error: {}", cause),
        },
    }
}
