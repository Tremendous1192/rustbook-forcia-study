fn main() {
    // Label for the outer loop, named 'main'
    'main: loop {
        // Print a message indicating the start of the main loop
        println!("main loop start");

        // Label for the inner loop, named 'sub'
        'sub: loop {
            // Print a message indicating the start of the sub loop
            println!("sub loop start");

            // Break out of the outer loop labeled 'main'
            break 'main;

            // The following code is unreachable due to the previous break statement
            println!("sub loop end");
        }

        // The following code is unreachable due to the previous break statement
        println!("main loop end");
    }
}
