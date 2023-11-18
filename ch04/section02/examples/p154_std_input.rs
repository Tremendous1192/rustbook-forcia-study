// ルートディレクトリ上のファイルの中身を読み込むプログラム
// 下記のコマンドで実行する
// cargo run --example p154_std_input -- input_p156.txt
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

        // Iterate over lines in the file and print each line
        for line in reader.lines() {
            let line = line.unwrap_or_else(|err| {
                eprintln!("Error reading line: {}", err);
                std::process::exit(1);
            });
            println!("{}", line);
        }
    } else {
        // Print a message if no file is specified
        println!("No file is specified.");
    }
}
