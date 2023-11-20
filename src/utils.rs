use std::{fs, io};


// Function to read file contents
pub fn read_file_contents(filename: &str) -> io::Result<String> {
    fs::read_to_string(filename)
}