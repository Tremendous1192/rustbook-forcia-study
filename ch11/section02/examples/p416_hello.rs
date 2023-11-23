// External C function declaration
extern "C" {
    fn c_hello();
}

// Main function
fn main() {
    // Print a Rust greeting message
    println!("Hello, world from Rust!");

    // Call the external C function
    unsafe {
        c_hello();
    }
}
