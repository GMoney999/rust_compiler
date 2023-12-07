mod lexer;
mod tests;
mod utils;
mod error;
mod parser;


use std::io::{Read};
use lexer::{tokenize};


fn main() {
    compile();
}


fn compile() {
    // fix_file_formatting();
    tokenize();
    // parse();
}

