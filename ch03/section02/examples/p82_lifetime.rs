// Main function where the program execution begins
fn main() {
    // Declare and initialize a mutable variable 'x' with the value 5
    let mut x = 5;

    // Create a reference 'y' pointing to the value of 'x'
    let y = &x;

    // Create a mutable reference 'z' pointing to the value of 'x'
    let z = &mut x;

    // Print the mutable reference 'z' and its value using the 'dbg!' macro
    dbg!(z);

    // Attempt to print 'x' after creating a mutable reference, which is not allowed
    // This line would result in a compilation error, so it's commented out
    dbg!(x);
}
