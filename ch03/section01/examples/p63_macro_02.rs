// Import the `Write` trait from the standard I/O library
use std::io::Write;

// Main function where the program execution begins
fn main() {
    // Print "hello" without a newline
    print!("hello");

    // Print "hello world" with a newline
    println!("hello {}", "world");

    // Print "hello error" to the standard error stream without a newline
    eprint!("hello {}", "error");

    // Print "hello" to the standard error stream with a newline
    eprintln!("hello");

    // Create a mutable vector `w` to store bytes
    let mut w = Vec::new();

    // Write "ABC" to the vector `w` using the `write!` macro
    write!(&mut w, "{}", "ABC");

    // Write " is 123" to the vector `w` with a newline using the `writeln!` macro
    writeln!(&mut w, " is 123");

    // Print the content of vector `w` using the `dbg!` macro
    dbg!(w);
}
