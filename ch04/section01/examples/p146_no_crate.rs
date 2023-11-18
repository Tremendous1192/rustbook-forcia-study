// このプログラムを実行する時は例えば下記のコマンドを入力する
// -- の後ろの"1 a xyz 2.0"が標準入力である
// cargo run --example p146_no_crate -- 1 a xyz 2.0

// Importing the 'env' module from the standard library
use std::env;

// Main function, entry point of the program
fn main() {
    // Collecting command-line arguments into a vector of strings
    let args: Vec<String> = env::args().collect();

    // Printing the collected command-line arguments
    println!("{:?}", args);
}
