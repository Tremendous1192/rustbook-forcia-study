// Main function where the program execution begins
fn main() {
    // Create a string variable 'important_data' and initialize it with "Hello, World!"
    let important_data = "Hello, World!".to_string();

    // Call the 'calc_data' function, passing a reference to 'important_data'
    calc_data(&important_data);

    // Print the original 'important_data'
    println!("{}", important_data);
}

// Function to perform some calculation on the reference to 'data' without taking ownership
fn calc_data(data: &String) {
    // Print the content of the referenced 'data'
    println!("{}", data);
}
