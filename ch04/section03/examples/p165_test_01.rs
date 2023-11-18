// テストモジュールの練習プログラム
// 下記のコマンドで実行する
// cargo test --example p165_test_01

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

    // Evaluation function for RpnCalculator, parsing a formula and calling eval_inner
    pub fn eval(&self, formula: &str) -> i32 {
        // Split the formula into tokens and reverse them for easy pop
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        // Call the eval_inner function with the tokens
        self.eval_inner(&mut tokens)
    }

    // Inner evaluation function for RpnCalculator
    fn eval_inner(&self, tokens: &mut Vec<&str>) -> i32 {
        // Create a stack to hold intermediate results
        let mut stack = Vec::new();

        // Process each token in the reversed order
        while let Some(token) = tokens.pop() {
            // Check if the token can be parsed as an integer
            if let Ok(x) = token.parse::<i32>() {
                // Push the integer onto the stack
                stack.push(x);
            } else {
                // If it's an operator, pop two values from the stack and perform the operation
                let y = stack.pop().expect("invalid syntax");
                let x = stack.pop().expect("invalid syntax");
                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => panic!("invalid token"),
                };
                // Push the result back onto the stack
                stack.push(res);
            }
            // Print the current state of tokens and stack if in verbose mode
            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }

        // If there's exactly one value on the stack, return it; otherwise, panic
        if stack.len() == 1 {
            stack[0]
        } else {
            panic!("invalid syntax")
        }
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

        // Print the line and the calculated result
        println!("{} = {}", line, answer);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        assert_eq!(2 * 2, 4);
    }
}
