// Main function where the program execution begins
fn main() {
    // Print the file name where this code is defined
    println!("defined in file: {}", file!());

    // Print the line number where this code is defined
    println!("defined on line: {}", line!());

    // Check if the target platform is Unix and print the result
    println!("is test: {}", cfg!(unix));

    // Print the value of the CARGO_HOME environment variable
    println!("CARGO_HOME: {}", env!("CARGO_HOME"));
}
