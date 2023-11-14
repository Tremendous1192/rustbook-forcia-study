fn main() {
    // Declare a variable count of type i32
    let count: i32;

    // Use a for loop to iterate over the range 0 to 9 (exclusive)
    for count in 0..10 {
        // Print the current value of count within the loop
        println!("count: {}", count);
    }

    // Create an array containing the values 0 to 9
    let array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    // Use a for loop to iterate over each element in the array
    for element in &array {
        // Print the current element within the loop
        println!("element: {}", element);
    }
}
