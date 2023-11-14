fn main() {
    // Create an instance of the `Iter` struct with initial values
    let it = Iter {
        current: 0,
        max: 10,
    };

    // Iterate over the values produced by the `Iter` iterator
    for num in it {
        // Print each number obtained from the iterator
        println!("num: {}", num);
    }
}

// Define a custom iterator struct called `Iter`
struct Iter {
    current: usize, // Current value of the iterator
    max: usize,     // Maximum value until which the iterator produces values
}

// Implement the `Iterator` trait for the custom `Iter` iterator
impl Iterator for Iter {
    // Specify that the iterator produces values of type `usize`
    type Item = usize;

    // Define the `next` method to control the iteration
    fn next(&mut self) -> Option<usize> {
        // Increment the current value
        self.current += 1;

        // Check if the current value is within the specified maximum
        if self.current - 1 < self.max {
            // Return the current value as `Some` if within the limit
            Some(self.current - 1)
        } else {
            // Return `None` when the iteration is complete
            None
        }
    }
}
