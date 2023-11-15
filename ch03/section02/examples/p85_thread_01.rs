// Import the 'thread' module from the standard library
use std::thread;

// Main function where the program execution begins
fn main() {
    // Spawn a new thread using the 'spawn' function from the 'thread' module
    thread::spawn(|| {
        // This closure is the code that will run in the new thread
        println!("Hello, world!");
    });

    // Note: The main thread continues its execution independently of the spawned thread
}
