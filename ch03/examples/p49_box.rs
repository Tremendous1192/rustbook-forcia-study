fn main() {
    // Create a byte array with the elements 'h', 'e', 'l', 'l', 'o'
    let byte_array = [b'h', b'e', b'l', b'l', b'o'];
    // Call the print function with the byte array wrapped in a Box
    print(Box::new(byte_array));

    // Create another byte array with the elements 'w', 'o', 'r', 'l', 'd', '!'
    let byte_array = [b'w', b'o', b'r', b'l', b'd', b'!'];
    // Call the print function with the second byte array wrapped in a Box
    print(Box::new(byte_array));
}

// Define a function print that takes a Box containing a byte array and prints it
fn print(s: Box<[u8]>) {
    // Print the byte array using debug formatting
    println!("{:?}", s);
}
