fn get_int_from_file() -> Result<i32, String> {
    // Specify the file path
    let path = "number.txt";

    // Read the file content into a string, converting I/O errors to a String in the Result
    let num_str = std::fs::read_to_string(path).map_err(|e| e.to_string())?;

    // Parse the string as an integer, multiplying the result by 2, or convert parsing errors to a String
    num_str
        .trim()
        .parse::<i32>()
        .map(|t| t * 2)
        .map_err(|e| e.to_string())
}

fn main() {
    // Match on the Result returned by get_int_from_file
    match get_int_from_file() {
        // If successful, print the result
        Ok(x) => println!("{}", x),
        // If an error occurred, print the error message
        Err(e) => println!("{}", e),
    }
}
