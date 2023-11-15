// Import the 'thread' module from the standard library
use std::thread;

// Main function where the program execution begins
fn main() {
    // Spawn a new thread using the 'spawn' function from the 'thread' module
    // The closure contains the code that will run in the new thread
    let handle = thread::spawn(|| {
        println!("Hello, world!");
    });

    // Debug print the thread handle
    dbg!(handle);
    // Note: The main thread continues its execution independently of the spawned thread
}
