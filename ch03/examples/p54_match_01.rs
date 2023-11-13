fn main() {
    // Declare and initialize an integer variable i with the value 1
    let i: i32 = 1;

    // Match the value of i against various patterns
    match i {
        // Check if i matches the pattern 1
        1 => println!("1"),

        // Check if i matches the pattern 2
        2 => println!("2"),

        // Check if i matches the pattern 3
        3 => println!("3"),

        // If none of the above patterns match, use the wildcard (_) to handle other cases
        _ => println!("misc"),
    }
}
