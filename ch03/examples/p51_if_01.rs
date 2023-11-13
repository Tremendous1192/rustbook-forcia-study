fn main() {
    // Declare a variable number with the value 1
    let number = 1;

    // Check if 0 is less than number
    if 0 < number {
        // Print a message if the condition is true
        println!("0 < number");
    } else if number < 0 {
        // Print a message if the first condition is false and number is less than 0
        println!("number < 0");
    } else {
        // Print a message if both conditions are false, i.e., 0 == number
        println!("0 == number");
    }
}
