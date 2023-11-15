// Import the 'thread' module from the standard library
use std::thread;

// Main function where the program execution begins
fn main() {
    // Create a vector to hold thread handles
    let mut handles = Vec::new();

    // Iterate from 0 to 9 (inclusive)
    for x in 0..10 {
        // Spawn a new thread for each iteration, using 'move' to capture 'x'
        // The closure contains the code that will run in each spawned thread
        handles.push(thread::spawn(move || {
            // Print a message with the current value of 'x'
            println!("Hello, world!: {}", x);
        }));
    }

    // Iterate over the handles and wait for each thread to finish ('join')
    for handle in handles {
        let _ = handle.join();
    }
}
