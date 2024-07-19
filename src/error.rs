use std::fmt;

use crate::token::Token;

pub mod error {
    pub fn report_error(loc: usize, message: &str) {
        eprintln!("[line {}] Error: {}", loc, message);
    }
}

#[derive(Debug, Clone)]
pub enum ParserError {
    UnmatchedParens(usize, String),
    UnexpectedToken(usize, String),
    InvalidExpression(usize, String),
    UnexpectedEndOfFile,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParserError::UnmatchedParens(line, loc) => write!(f, "[line {}] ParserError: Unmatched Parentheses at {}", line, loc),
            ParserError::UnexpectedToken(line, loc) => write!(f, "[line {}] ParserError: Unexpected Token at {}", line, loc),
            ParserError::InvalidExpression(line, loc) => write!(f, "[line {}] ParserError: Invalid Expression at {}", line, loc),
            ParserError::UnexpectedEndOfFile => write!(f, "ParserError: Unexpected End of File"),
        }
    }
}


#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub token : Token,
    pub msg : String,
}

impl RuntimeError {
    pub fn new(token: Token, msg: String) -> RuntimeError {
        RuntimeError {
            token,
            msg,
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RuntimeError: {} at {}", self.msg, self.token.lexeme)
    }
}