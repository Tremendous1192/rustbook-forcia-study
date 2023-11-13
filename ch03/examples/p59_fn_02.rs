fn main() {
    // Define and initialize the variable `input` with the value -1
    let input = -1;

    // Print the value of the `input` variable
    println!("input: {}", input);

    // Call the `abs` function with the `input` value and store the result in `result`
    let result = abs(input);

    // Print the result of the `abs` function
    println!("result: {}", result);
}

// Define a function named `abs` that takes an integer `number` and returns its absolute value
fn abs(number: i32) -> i32 {
    // Check if the number is negative, and if so, return its negation; otherwise, return the number as is
    if number < 0 {
        return -number;
    }
    number
}
