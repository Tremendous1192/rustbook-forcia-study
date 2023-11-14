// Main function where the program execution begins
fn main() {
    // Create a mutable string variable 'important_data' and initialize it with "Hello, World!"
    let mut important_data = "Hello, World!".to_string();

    // Call the 'calc_data' function, passing 'important_data' as an argument
    important_data = calc_data(important_data);

    // Print the modified 'important_data'
    println!("{}", important_data);
}

// Function to perform some calculation on the input 'data' and return the result
fn calc_data(data: String) -> String {
    // Print the original 'data'
    println!("{}", data);

    // Return the original 'data', demonstrating ownership transfer and borrowing
    data
}
