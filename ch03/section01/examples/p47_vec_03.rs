fn main() {
    // Create a vector v with elements 1, 2, 3, 4, 5
    let v = vec![1, 2, 3, 4, 5];

    // Iterate over the elements of the vector using a for loop
    for element in &v {
        // Print each element in the vector
        println!("{}", element);
    }
}
