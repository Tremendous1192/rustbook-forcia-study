// Main function where the program execution begins
fn main() {
    // Declare and initialize a variable 'x' with the value 5
    let x = 5;

    // Create references 'y' and 'z' pointing to the value of 'x'
    let y = &x;
    let z = &x;

    // Print the value of 'x' using the 'dbg!' macro, which includes source location
    dbg!(x);

    // Print the reference 'y' and its value
    dbg!(y);

    // Print the reference 'z' and its value
    dbg!(z);
}
