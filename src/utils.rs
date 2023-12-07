use std::{env, fs, io, process};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;


// Function to read file contents
pub fn read_file_contents(filename: &str) -> io::Result<String> {
    fs::read_to_string(filename)
}

pub fn is_number(token: &str) -> bool {
    token.chars().all(|c| c.is_digit(10))
}

pub fn is_identifier(token: &str) -> bool {
    let mut chars = token.chars();
    match chars.next() {
        Some(first) => (first.is_alphabetic() || first == '_') && chars.all(|c| c.is_alphanumeric() || c == '_'),
        None => false,
    }
}

pub fn is_reserved_word(token: &str) -> bool {
    let reserved = vec!["program", "var", "begin", "end.", "integer", "write"];
    reserved.contains(&token)
}

// This function takes in the name of a file and returns a buffer reader to the file
// Calling this function: let file_reader = open_file("path/to/myfile.txt")?;
pub fn open_file(file_name: &str) -> io::Result<BufReader<File>> {
    let path = Path::new(file_name);
    let file = File::open(&path)?;
    Ok(io::BufReader::new(file))
}


//////////////// REFORMATTING FILE //////////////////
pub fn remove_comments(contents: &str) -> String {
    let mut result = String::new();
    let mut in_comment = false;
    let mut chars = contents.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '(' if chars.peek() == Some(&'*') => {
                in_comment = true;
                chars.next(); // Skip the '*' character
            }
            '*' if chars.peek() == Some(&')') => {
                in_comment = false;
                chars.next(); // Skip the ')' character
            }
            _ if !in_comment => result.push(ch),
            _ => {}
        }
    }

    result
}


pub fn remove_excess_whitespaces(contents: &str) -> String {
    contents
        .lines()
        .map(|line| {
            let mut in_string = false;
            let mut new_line = String::new();
            let mut chars = line.chars().peekable();

            while let Some(ch) = chars.next() {
                if ch == '"' { in_string = !in_string; }
                if ch.is_whitespace() && !in_string && chars.peek().map_or(false, |&next| next.is_whitespace()) {
                    continue;
                }
                new_line.push(ch);
            }

            new_line
        })
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<String>>()
        .join("\n")
}


pub fn fix_file_formatting() {
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

// If an identifier contains any letters besides the ones in our grammar, panic with the message "Invalid identifier"
pub fn validate_identifier(word: &str) {
    for c in word.chars() {
        match c {
            'a' | 'b' | 'c' | 'd' | 'w' | 'f' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => continue,
            _ => panic!("Invalid identifier"),
        }
    }
}