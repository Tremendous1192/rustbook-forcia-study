fn main() {
    // Initialize a mutable variable count with the value 0
    let mut count = 0;

    // Start an infinite loop using 'loop'
    let result = loop {
        // Print the current value of count
        println!("count: {}", count);

        // Increment count by 1
        count += 1;

        // Check if count is equal to 10
        if count == 10 {
            // If count is 10, exit the loop and assign the value of count to result
            break count;
        }
    };

    // Print the result after exiting the loop
    println!("result: {}", result);
}
