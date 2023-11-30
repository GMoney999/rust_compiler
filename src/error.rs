// error.rs: Compiler Error Handling Module
//
// This module defines custom error types for the compiler, including ExpectedKeyword and ExpectedToken enums.
// These error types represent specific parsing and lexical analysis errors encountered during compilation.
// Each error type implements the std::fmt::Display trait, allowing for formatted, human-readable error messages that clearly communicate the nature of the error to the user.

use std::fmt;
use std::fmt::Formatter;

enum ExpectedKeyword {
    ExpectedKeywordProgram,
    ExpectedKeywordVar,
    ExpectedKeywordBegin,
    ExpectedKeywordEnd,
    ExpectedKeywordInteger,
    ExpectedKeywordWrite,
}
impl fmt::Display for ExpectedKeyword {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ExpectedKeyword::ExpectedKeywordProgram => write!(f, "Keyword 'program' is expected."),
            ExpectedKeyword::ExpectedKeywordVar => write!(f, "Keyword 'var' is expected."),
            ExpectedKeyword::ExpectedKeywordBegin => write!(f, "Keyword 'begin' is expected."),
            ExpectedKeyword::ExpectedKeywordEnd => write!(f, "Keyword 'end' is expected"),
            ExpectedKeyword::ExpectedKeywordInteger => write!(f, "Keyword 'integer' is expected."),
            ExpectedKeyword::ExpectedKeywordWrite => write!(f, "Keyword 'write' is expected."),
        }
    }
}

enum ExpectedToken {
    ExpectedTokenSemicolon,
    ExpectedTokenComma,
    ExpectedTokenPeriod,
    ExpectedTokenLeftParentheses,
    ExpectedTokenRightParentheses
}
impl fmt::Display for ExpectedToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ExpectedToken::ExpectedTokenSemicolon => write!(f, "';' is missing."),
            ExpectedToken::ExpectedTokenComma => write!(f, "',' is missing."),
            ExpectedToken::ExpectedTokenPeriod => write!(f, "'.' is missing."),
            ExpectedToken::ExpectedTokenLeftParentheses => write!(f, "Left parentheses is missing."),
            ExpectedToken::ExpectedTokenRightParentheses => write!(f, "Right parentheses is missing."),
        }
    }
}