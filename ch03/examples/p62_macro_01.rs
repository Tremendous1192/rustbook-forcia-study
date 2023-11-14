// Main function where the program execution begins
fn main() {
    // Concatenate literals "A", "b2", and the integer 3 into a static string `s`
    let s = concat!("A", "b2", 3);
    // Print the concatenated string
    println!("{}", s);

    // Format a new string by combining the previous string `s` with a tuple ("D", 5)
    let s = format!("{}-{:?}", s, ("D", 5));
    // Print the formatted string
    println!("{}", s);

    // Format a new string by concatenating "abc" and "def"
    let s = format!("{}{}", "abc", "def");
    // Print the formatted string
    println!("{}", s);
}
