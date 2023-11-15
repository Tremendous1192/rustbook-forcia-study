// Import necessary modules from the standard library
use std::sync::{Arc, Mutex};
use std::thread;

// Main function where the program execution begins
fn main() {
    // Create a vector to hold thread handles
    let mut handles = Vec::new();

    // Create shared data wrapped in an Arc (atomic reference counter) and a Mutex
    let data = Arc::new(Mutex::new(vec![1; 10]));

    // Iterate from 0 to 9 (inclusive)
    for x in 0..10 {
        // Clone the Arc to ensure each thread has its own reference to the shared data
        let data_ref = data.clone();

        // Spawn a new thread for each iteration, using 'move' to capture 'x'
        // The closure contains the code that will run in each spawned thread
        handles.push(thread::spawn(move || {
            // Acquire the lock on the Mutex, modifying the shared data
            let mut data = data_ref.lock().unwrap();
            data[x] += 1; // Increment the value at index 'x'
        }));
    }

    // Iterate over the handles and wait for each thread to finish ('join')
    for handle in handles {
        let _ = handle.join();
    }

    // Print the final state of the shared data using 'dbg!'
    dbg!(data);
}
