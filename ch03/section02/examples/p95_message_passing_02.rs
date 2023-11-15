// Import necessary modules from the standard library
use std::sync::mpsc;
use std::thread;

// Main function where the program execution begins
fn main() {
    // Vector to store thread handles
    let mut handles = Vec::new();

    // Initial data vector with 10 elements, each initialized to 1
    let mut data = vec![1; 10];

    // Vectors to store sender and receiver channels for communication
    let mut snd_channels = Vec::new();
    let mut rcv_channels = Vec::new();

    // Loop to create 10 threads, each with its own sender and receiver channel
    for _ in 0..10 {
        // Create sender and receiver channels
        let (snd_tx, snd_rx) = mpsc::channel();
        let (rcv_tx, rcv_rx) = mpsc::channel();

        // Store sender and receiver channels in vectors
        snd_channels.push(snd_tx);
        rcv_channels.push(rcv_rx);

        // Spawn a thread for each iteration
        handles.push(thread::spawn(move || {
            // Receive data from sender channel, increment it, and send it to receiver channel
            let mut data = snd_rx.recv().unwrap();
            data += 1;
            let _ = rcv_tx.send(data);
        }));
    }

    // Loop to send data from the main thread to the spawned threads
    for x in 0..10 {
        let _ = snd_channels[x].send(data[x]);
    }

    // Loop to receive modified data from the spawned threads back to the main thread
    for x in 0..10 {
        data[x] = rcv_channels[x].recv().unwrap();
    }

    // Wait for all spawned threads to finish
    for handle in handles {
        let _ = handle.join();
    }

    // Print the modified data
    dbg!(data);
}
