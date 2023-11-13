fn main() {
    // Call the `add` function with arguments 1 and 2 and store the result in `x`
    let x = add(1, 2);

    // Print the value of `x`
    println!("x = {}", x);
}

// Define a function named `add` that takes two parameters and returns their sum
fn add(a: i32, b: i32) -> i32 {
    // Return the sum of the two parameters
    a + b
}
