fn get_int_from_file() -> i32 {
    // Specify the file path
    let path = "number.txt";

    // Read the file content into a string, or panic if it fails
    let num_str = std::fs::read_to_string(path).expect("failed to open the file");

    // Parse the string as an integer, or panic if parsing fails
    let ret = num_str
        .trim()
        .parse::<i32>()
        .expect("failed to parse string to a number.");

    // Multiply the parsed number by 2 and return the result
    ret * 2
}

fn main() {
    // Print the result of the get_int_from_file function
    println!("{}", get_int_from_file());
}
