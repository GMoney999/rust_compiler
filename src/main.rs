mod lexer;
mod tests;
mod utils;
mod error;


use std::env;
use std::fs;
use std::io::{self, Read};
use std::process;
use utils::{read_file_contents};
use lexer::{remove_comments, remove_excess_whitespaces};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <input_filename> <output_filename>", args[0]);
        process::exit(1);
    }

    let input_filename = &args[1];
    let output_filename = &args[2];

    // Read input file
    let contents = read_file_contents(input_filename)
        .expect("Failed to read input file");

    // Process contents
    let processed = remove_excess_whitespaces(&remove_comments(&contents));

    // Write to output file
    fs::write(output_filename, processed)
        .expect("Failed to write to output file");

    println!("Processing complete. Output written to {}", output_filename);
}


