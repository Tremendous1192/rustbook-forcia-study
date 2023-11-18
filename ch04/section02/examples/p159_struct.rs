// ルートディレクトリ上のファイルの中身を読み込むプログラム
// 下記のコマンドで実行する
// cargo run --example p159_struct -- input_p156.txt

// Import the required libraries
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Define a struct RpnCalculator to represent an RPN calculator
struct RpnCalculator(bool);

// Implementation block for RpnCalculator
impl RpnCalculator {
    // Constructor function for RpnCalculator
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    // Evaluation function for RpnCalculator, currently returning a placeholder value
    pub fn eval(&self, line: &str) -> i32 {
        0 // Placeholder implementation
    }
}

// Function to process the contents of the file using an RpnCalculator
fn run<R: BufRead>(reader: R, verbose: bool) {
    // Create an instance of RpnCalculator with the specified verbosity
    let calc = RpnCalculator::new(verbose);

    // Iterate over lines in the file and print each line (for now)
    for line in reader.lines() {
        let line = line.unwrap();

        // Call the eval function on the RpnCalculator instance
        let answer = calc.eval(&line);

        // Print the line (placeholder)
        println!("{}", line);
    }
}

// Define a struct Opts using the Parser trait from the clap crate
#[derive(Parser, Debug)]
#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "Your name",
    about = "Super awesome sample RPN calculator"
)]
struct Opts {
    // Define command-line options using clap attributes
    #[clap(short, long)]
    verbose: bool,

    #[clap(name = "FILE")]
    formula_file: Option<String>,
}

// Main function for the RPN calculator program
fn main() {
    // Parse command-line arguments into the Opts struct
    let opts = Opts::parse();

    // Check if a formula file is specified
    if let Some(path) = opts.formula_file {
        // Attempt to open the specified file
        let f = File::open(&path).unwrap_or_else(|err| {
            eprintln!("Error opening file '{}': {}", path, err);
            std::process::exit(1);
        });

        // Create a buffered reader for efficient reading
        let reader = BufReader::new(f);

        // Call the run function to process the file contents
        run(reader, opts.verbose);
    } else {
        // Print a message if no file is specified
        println!("No file is specified.");
    }
}
