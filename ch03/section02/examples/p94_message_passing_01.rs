// Import necessary modules from the standard library
use std::sync::mpsc;
use std::thread;

// Main function where the program execution begins
fn main() {
    // Create a multiple-producer, single-consumer channel
    let (tx, rx) = mpsc::channel();

    // Spawn a new thread to receive data from the channel
    let handle = thread::spawn(move || {
        // Receive data from the channel and print it
        let data = rx.recv().unwrap();
        println!("{}", data);
    });

    // Send a message through the channel
    let _ = tx.send("Hello, world!");

    // Wait for the spawned thread to finish
    let _ = handle.join();
}
