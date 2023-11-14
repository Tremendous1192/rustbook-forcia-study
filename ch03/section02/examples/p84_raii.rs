// Define a custom struct 'Droppable'
struct Droppable;

// Implement the 'Drop' trait for 'Droppable' to define custom behavior when the value is dropped
impl Drop for Droppable {
    fn drop(&mut self) {
        // Print a message when the value is dropped
        println!("Resource will be released!");
    }
}

// Main function where the program execution begins
fn main() {
    // Create a new scope with a block
    {
        // Declare a variable 'd' of type 'Droppable'
        let d = Droppable;
        // 'd' goes out of scope at the end of this block, and its 'drop' method is called
    }

    // Print a message indicating that the Droppable resource should be released at the end of the block
    println!("The Droppable should be released at the end of the block.");
    // At this point, the 'Drop' trait implementation for 'Droppable' is triggered, printing the release message
}
