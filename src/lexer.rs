use std::{fs};
use crate::utils::{validate_identifier};

//////////////// TOKENIZATION //////////////////
// Each variant of this enum represents a different kind of token that the lexer might encounter in an input string
#[derive(Debug, PartialEq, Clone)] // Debug for easy printing
pub enum TokenKind {
    Identifier(String),
    Keyword(String),
    StringLiteral(String),
    Number(i64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LeftParen,
    RightParen,
    Quote,
    SemiColon,
    Dot,
    Comma,
    Colon,
    Equal,
    Whitespace,
    Bad,
    Eof,
}

// Object to represent a span of text (literal) with a start and end
#[derive(Debug, PartialEq, Clone)]
pub struct TextSpan {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) literal: String
} impl TextSpan {
    // Constructor function
    pub fn new(start: usize, end: usize, literal: String) -> Self {
        Self { start, end, literal }
    }
    // Function that returns the length of the text span
    pub fn length(&self) -> usize {
        self.end - self.start
    }
}

//////////////////////// TOKEN STRUCT //////////////////////
#[derive(Debug, PartialEq, Clone)] // For comparing types of tokens to make decisions
pub struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) span: TextSpan,
} impl Token {
    // Constructor function
    pub fn new(kind: TokenKind, span: TextSpan) -> Self {
        Self { kind, span }
    }
}

//////////////////////// LEXER STRUCT //////////////////////
// Defines a struct for the lexer, which will process an input string while keeping track of its current position
pub struct Lexer<'a> {
    input: &'a str,
    current_pos: usize,

}
//////////////////////// LEXER METHODS //////////////////////
impl<'a> Lexer<'a> {
    // Constructor function
    pub fn new(input: &'a str) -> Self {
        Self { input, current_pos: 0 }
    }
    // This function gets the next token from the input
    pub fn next_token(&mut self) -> Option<Token> {
        // Check if we have reached the end of the input string
        if self.current_pos == self.input.len() {
            let eof_char: char = '\0'; // Represents end of file (null char)
            self.current_pos+=1; // Move the current position beyond the end of the input
            // Return Token representing the end of the file
            return Some(Token::new(
                TokenKind::Eof,
                TextSpan::new(0, 0, eof_char.to_string())
            ));
        }
        // Grab the current character
        let c = self.current_char();
        // The map function handles the Option type without explicitly handling the None case which we have already checked for above
        return c.map(|c| {
            let start = self.current_pos; // Sets the variable start to the current position, marking the beginning of the new token
            let mut kind = TokenKind::Bad; // Sets the kind of token to 'Bad' for now



            // Check if the current character is the start of a number
            if Self::is_number_start(&c) {
                // If the current character is the start of a number, consume the number from the input and store it in 'number'
                let number: i64 = self.consume_number();
                // Set the type of the Token to 'Number'
                kind = TokenKind::Number(number);



                // If the current character is not a number start, check if it's a whitespace character
            } else if Self::is_whitespace(&c) {
                // If the current character is a whitespace, consume the whitespace
                self.consume();
                // Set the type of token to 'Whitespace;
                kind = TokenKind::Whitespace;
                // If the current character is not a number or a whitespace, it must be punctuation
            } else if c.is_alphabetic() {
                // If the current character is alphabetic, consume the word from the input and store it in 'word'
                let mut word = String::new();
                while let Some(c) = self.current_char() {
                    if c.is_alphabetic() {
                        self.consume().unwrap();
                        word.push(c);
                    } else {
                        break;
                    }
                }
                // Check if the word is a reserved word
                if self.is_reserved_word(&word) {
                    // If the word is a reserved word, set the type of the Token to 'Keyword'
                    kind = TokenKind::Keyword(word);
                } else {
                    // If the word is not a reserved word, check if it is a valid identifier (contains only a, b, c, d, w, or f)
                    // If it an invalid identifier, panic with the message "Invalid identifier"
                    // validate_identifier(&word);
                    kind = TokenKind::Identifier(word);
                }

            } else if c == '"' {
                // If the current character is a quote, consume the string from the input and store it in 'string'
                let mut string = String::new();
                while let Some(c) = self.current_char() {
                    if c != '"' {
                        self.consume().unwrap();
                        string.push(c);
                    } else {
                        break;
                    }
                }
                // Set the type of the Token to 'StringLiteral'
                kind = TokenKind::StringLiteral(string);
            } else {
                // consume_punctuation matches the Token to its type of punctuation i.e. + - * / ( ) or Bad
                kind = self.consume_punctuation();
            }
            // Set the end of the token to the current position after consuming last character, marking the end of the Token
            let end = self.current_pos;
            // Extract the substring from the input string that corresponds to the token and convert it to a String.
            let literal = self.input[start..end].to_string();
            // Create a new TextSpan object with the start, end, and literal of the token
            let span = TextSpan::new(start, end, literal);
            // Create a new Token with the determined type and span and return it
            Token::new(kind, span)
        })
    }
    // This function gets the current character at current_pos
    fn current_char(&self) -> Option<char> {

        self.input.chars().nth(self.current_pos)
    }

    // This function consumes the current character and moves to the next position
    fn consume(&mut self) -> Option<char> {
        if self.current_pos >= self.input.len() {
            return None;
        }
        let c = self.current_char()?;
        let next_char_boundary = self.input[self.current_pos..].chars().next()?.len_utf8();
        self.current_pos += next_char_boundary;
        Some(c)
    }
    // This function consumes a sequence of digits and constructs a number
    fn consume_number(&mut self) -> i64 {
        let mut number: i64 = 0;
        while let Some(c) = self.current_char() {
            if c.is_digit(10) {
                self.consume().unwrap();
                number = number * 10 + c.to_digit(10).unwrap() as i64;
            } else {
                break;
            }
        }
        number
    }
    // This function consumes punctuation characters and returns the corresponding TokenKind
    fn consume_punctuation(&mut self) -> TokenKind {
        let c = self.consume().unwrap();
        match c {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Asterisk,
            '/' => TokenKind::Slash,
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '"' => TokenKind::Quote,
            ';' => TokenKind::SemiColon,
            '.' => TokenKind::Dot,
            ',' => TokenKind::Comma,
            ':' => TokenKind::Colon,
            '=' => TokenKind::Equal,
            _ => TokenKind::Bad,
        }
    }
    // Helper function to determine if a character can start a number
    fn is_number_start(c: &char) -> bool {
        c.is_digit(10)
    }
    // Helper function to check if a character is whitespace
    fn is_whitespace(c: &char) -> bool {
        c.is_whitespace()
    }

    // Helper function to check if a word is valid
    fn is_reserved_word(&self, word: &str) -> bool {
        // If the word is a reserved word, return true
        if word == "program" || word == "var" || word == "begin" || word == "end." || word == "integer" || word == "write" {
            return true;
        }
        return false;
    }


}

pub fn tokenize() {
    let input_file = fs::read_to_string("finalv2.txt");
    let binding = input_file.unwrap();
    let mut lexer = Lexer::new(binding.as_str());
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }
    println!("{:?}", tokens);
}






