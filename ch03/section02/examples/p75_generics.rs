// Function 'make_tuple' that takes two generic parameters and returns a tuple of those parameters
fn make_tuple<T, S>(t: T, s: S) -> (T, S) {
    (t, s) // Returns a tuple containing the values of the parameters
}

// Main function where the program execution begins
fn main() {
    // Create a tuple 't1' by calling 'make_tuple' with integer arguments 1 and 2
    let t1 = make_tuple(1, 2);
    println!("{:?}", t1); // Print the tuple 't1'

    // Create a tuple 't2' by calling 'make_tuple' with string arguments "Hello" and "World!"
    let t2 = make_tuple("Hello", "World!");
    println!("{:?}", t2); // Print the tuple 't2'

    // Create a tuple 't3' by calling 'make_tuple' with vector arguments [1, 2, 3] and [4, 5]
    let t3 = make_tuple(vec![1, 2, 3], vec![4, 5]);
    println!("{:?}", t3); // Print the tuple 't3'

    // Create a tuple 't4' by calling 'make_tuple' with mixed-type arguments 3 and "years old"
    let t4 = make_tuple(3, "years old");
    println!("{:?}", t4); // Print the tuple 't4'
}
