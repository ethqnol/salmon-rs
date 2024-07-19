use crate::error::error::report_error;
use crate::token::{Token, TokenType};
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    source: Peekable<Chars<'a>>, // Use Peekable<Chars<'a>> for peeking
    current_char: Option<char>,
    current_line: usize,
    current_column: usize,
    tokens: Vec<Token>,
    pub num_errors: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut lexer = Lexer {
            source: source.chars().peekable(), // Convert to Peekable<Chars<'a>>
            current_char: None,
            current_line: 1,
            current_column: 1,
            tokens: Vec::new(),
            num_errors: 0,
        };
        lexer.current_char = lexer.source.next();
        lexer
    }

    pub fn get_tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        while let Some(c) = self.current_char {
            let token = match c {
                '(' => Some(Token::new(
                    TokenType::LEFT_PAREN,
                    c.to_string(),
                    self.current_line,
                )),
                ')' => Some(Token::new(
                    TokenType::RIGHT_PAREN,
                    c.to_string(),
                    self.current_line,
                )),
                '{' => Some(Token::new(
                    TokenType::LEFT_BRACE,
                    c.to_string(),
                    self.current_line,
                )),
                '}' => Some(Token::new(
                    TokenType::RIGHT_BRACE,
                    c.to_string(),
                    self.current_line,
                )),
                '*' => Some(Token::new(
                    TokenType::STAR,
                    c.to_string(),
                    self.current_line,
                )),
                '.' => Some(Token::new(TokenType::DOT, c.to_string(), self.current_line)),
                ',' => Some(Token::new(
                    TokenType::COMMA,
                    c.to_string(),
                    self.current_line,
                )),
                '+' => Some(Token::new(
                    TokenType::PLUS,
                    c.to_string(),
                    self.current_line,
                )),
                ';' => Some(Token::new(
                    TokenType::SEMICOLON,
                    c.to_string(),
                    self.current_line,
                )),
                '-' => Some(Token::new(
                    TokenType::MINUS,
                    c.to_string(),
                    self.current_line,
                )),
                '=' => {
                    if self.match_next('=') {
                        self.advance();
                        Some(Token::new(
                            TokenType::EQUAL_EQUAL,
                            "==".to_string(),
                            self.current_line,
                        ))
                    } else {
                        Some(Token::new(
                            TokenType::EQUAL,
                            c.to_string(),
                            self.current_line,
                        ))
                    }
                }
                '!' => {
                    if self.match_next('=') {
                        self.advance();
                        Some(Token::new(
                            TokenType::BANG_EQUAL,
                            "!=".to_string(),
                            self.current_line,
                        ))
                    } else {
                        Some(Token::new(
                            TokenType::BANG,
                            c.to_string(),
                            self.current_line,
                        ))
                    }
                }
                '>' => {
                    if self.match_next('=') {
                        self.advance();
                        Some(Token::new(
                            TokenType::GREATER_EQUAL,
                            ">=".to_string(),
                            self.current_line,
                        ))
                    } else {
                        Some(Token::new(
                            TokenType::GREATER,
                            c.to_string(),
                            self.current_line,
                        ))
                    }
                }
                '<' => {
                    if self.match_next('=') {
                        self.advance();
                        Some(Token::new(
                            TokenType::LESS_EQUAL,
                            "<=".to_string(),
                            self.current_line,
                        ))
                    } else {
                        Some(Token::new(
                            TokenType::LESS,
                            c.to_string(),
                            self.current_line,
                        ))
                    }
                }

                '/' => {
                    if self.match_next('/') {
                        while let Some(c) = self.current_char {
                            if c == '\n' {
                                break;
                            }
                            self.advance();
                        }
                        None
                    } else {
                        Some(Token::new(
                            TokenType::SLASH,
                            c.to_string(),
                            self.current_line,
                        ))
                    }
                }

                '"' => {
                    let mut value = String::new();
                    let mut seen_end_quote: bool = false;
                    self.advance();
                    while let Some(c) = self.current_char {
                        if c == '"' {
                            seen_end_quote = true;
                            break;
                        }
                        value.push(c);
                        self.advance();
                    }
                    if seen_end_quote == false {
                        report_error(self.current_line, "Unterminated string.");
                        self.num_errors += 1;
                        None
                    } else {
                        Some(Token::new(TokenType::STRING, value, self.current_line))
                    }
                }
                ' ' | '\r' | '\t' | '\n' => None,
                _ => {
                    if c.is_ascii_digit() {
                        let mut seen_decimal: bool = false;
                        let mut value = String::from(c);

                        while let Some(&next_char) = self.source.peek() {
                            if next_char.is_ascii_digit() {
                                value.push(next_char);
                                self.advance();
                            } else if next_char == '.' && seen_decimal == false {
                                seen_decimal = true;
                                value.push(next_char);
                                self.advance();
                            } else {
                                break;
                            }
                        }

                        if value.ends_with(".") {
                            value.push_str("0");
                            self.tokens.push(Token::new(
                                TokenType::NUMBER,
                                value,
                                self.current_line,
                            ));
                            Some(Token::new(
                                TokenType::DOT,
                                ".".to_string(),
                                self.current_line,
                            ))
                        } else {
                            Some(Token::new(TokenType::NUMBER, value, self.current_line))
                        }
                    } else if c.is_alphabetic() || c == '_' {
                        let mut value = String::from(c);
                        while let Some(&next_char) = self.source.peek() {
                            if next_char.is_alphanumeric() || next_char == '_' {
                                value.push(next_char);
                                self.advance();
                            } else {
                                break;
                            }
                        }
                        match value.as_str() {
                            "and" => Some(Token::new(TokenType::AND, value, self.current_line)),
                            "class" => Some(Token::new(TokenType::CLASS, value, self.current_line)),
                            "else" => Some(Token::new(TokenType::ELSE, value, self.current_line)),
                            "false" => Some(Token::new(TokenType::FALSE, value, self.current_line)),
                            "for" => Some(Token::new(TokenType::FOR, value, self.current_line)),
                            "fn" => Some(Token::new(TokenType::FN, value, self.current_line)),
                            "if" => Some(Token::new(TokenType::IF, value, self.current_line)),
                            "null" => Some(Token::new(TokenType::NULL, value, self.current_line)),
                            "or" => Some(Token::new(TokenType::OR, value, self.current_line)),
                            "print" => Some(Token::new(TokenType::PRINT, value, self.current_line)),
                            "return" => {
                                Some(Token::new(TokenType::RETURN, value, self.current_line))
                            }
                            "crate" => Some(Token::new(TokenType::SUPER, value, self.current_line)),
                            "this" => Some(Token::new(TokenType::THIS, value, self.current_line)),
                            "true" => Some(Token::new(TokenType::TRUE, value, self.current_line)),
                            "var" => Some(Token::new(TokenType::VAR, value, self.current_line)),
                            "while" => Some(Token::new(TokenType::WHILE, value, self.current_line)),
                            _ => Some(Token::new(TokenType::IDENTIFIER, value, self.current_line)),
                        }
                    } else {
                        report_error(
                            self.current_line,
                            format!("Unexpected character: {}", c.to_string()).as_str(),
                        );
                        self.num_errors += 1;
                        None
                    }
                }
            };

            if let Some(token) = token {
                self.tokens.push(token);
            }

            self.advance();
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            self.current_line,
        ));
        self.tokens.clone()
    }

    fn match_next(&mut self, expected: char) -> bool {
        if let Some(next) = self.source.peek() {
            if *next == expected {
                return true;
            }
        }
        return false;
    }

    fn advance(&mut self) -> () {
        if self.current_char == Some('\n') {
            self.current_line += 1;
        }
        self.current_column += 1;
        self.current_char = self.source.next();
    }
}
