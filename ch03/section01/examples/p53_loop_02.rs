fn main() {
    // Initialize a mutable variable count with the value 0
    let mut count = 0;

    // Execute the following block while the condition count < 10 is true
    while count < 10 {
        // Print the current value of count
        println!("count: {}", count);

        // Increment count by 1
        count += 1;
    }

    // Print the final value of count after the loop
    println!("count: {}", count);
}
