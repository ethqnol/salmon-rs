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
pub enum RuntimeError {
    InvalidBinaryOperation(Token, String),
    InvalidUnaryOperation(Token, String),
    InvalidOperandType(Token, String),
    UndefinedVariable(Token),
    InvalidLiteral(Token, String),
    InvalidLogicalOperation(Token, String),
    InvalidFunctionCall(Token, String),
    
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RuntimeError::InvalidBinaryOperation(token, msg) => write!(f, "RuntimeError: Invalid Binary Operation at {}. {}", token.lexeme, msg),
            RuntimeError::InvalidUnaryOperation(token, msg) => write!(f, "RuntimeError: Invalid Unary Operation at {}. {}", token.lexeme, msg),
            RuntimeError::InvalidOperandType(token, msg) => write!(f, "RuntimeError: Invalid Operand Type at {}. {}", token.lexeme, msg),
            RuntimeError::UndefinedVariable(token) => write!(f, "RuntimeError: Undefined Variable at {}.", token.lexeme),
            RuntimeError::InvalidLiteral(token, msg) => write!(f, "RuntimeError: Invalid Literal at {}. {}", token.lexeme, msg),
            RuntimeError::InvalidLogicalOperation(token, msg) => write!(f, "RuntimeError: Invalid Logical Operation at {}. {}", token.lexeme, msg),
            RuntimeError::InvalidFunctionCall(token, msg) => write!(f, "RuntimeError: Invalid Function Call at {}. {}", token.lexeme, msg),
        }
    }
}