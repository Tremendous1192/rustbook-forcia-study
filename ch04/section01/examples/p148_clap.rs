// このプログラムを実行する時は例えば下記のコマンドを入力する
// -- の後ろのファイル名が標準入力である
// cargo run --example p148_clap  -- p146_no_crate.rs
//-Vの場合、全てのファイル名を取得するようです
// cargo run --example p148_clap  -- -V

// Importing necessary items from the 'clap' crate
use clap::{arg, App};

// Main function, entry point of the program
fn main() {
    // Creating a new command-line application using the 'clap' crate
    let matches = App::new("My RPN program")
        .version("1.0.0")
        .author("Your name")
        .about("Super awesome sample RPN calculator")
        // Defining command-line arguments using the 'arg!' macro
        .arg(arg!([FILE] "Formulas written in RPN").required(false))
        .arg(arg!(-v --verbose ... "Sets the level of verbosity").required(false))
        .get_matches();

    // Extracting and printing the value of the "FILE" argument
    match matches.value_of("FILE") {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified."),
    }

    // Checking and printing whether the "verbose" flag is present
    let verbose = matches.is_present("verbose");
    println!("Is verbosity specified?: {}", verbose);
}
