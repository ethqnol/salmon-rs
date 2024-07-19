use crate::error::ParserError;
use crate::ast::expr::Expr;
use crate::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    pub error_count: u32,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            current: 0,
            error_count: 0,
        }
    }

    fn peek(&self) -> Option<Token> {
        if self.current < self.tokens.len() {
            return Some(self.tokens[self.current].clone());
        }
        None
    }

    fn view_prev(&self) -> Option<Token> {
        if self.current > 0 {
            return Some(self.tokens[self.current - 1].clone());
        }
        None
    }

    fn is_end(&self) -> bool {
        if let Some(token) = self.peek() {
            return token.token_type == TokenType::EOF;
        } else {
            if self.current < self.tokens.len() {
                return false;
            } else {
                return true;
            }
        }
    }

    fn consume(
        &mut self,
        token_type: TokenType,
        if_error_msg: String,
    ) -> Result<Token, ParserError> {
        if self.check_type(token_type) {
            return Ok(self.advance().unwrap());
        }

        self.error_count += 1;
        Err(ParserError::UnexpectedToken(
            self.view_prev().unwrap().line,
            if_error_msg,
        ))
    }

    fn advance(&mut self) -> Option<Token> {
        if !self.is_end() {
            self.current += 1;
        }
        self.view_prev().clone()
    }

    fn check_type(&self, expected_type: TokenType) -> bool {
        if let Some(token) = self.peek() {
            if token.token_type == expected_type {
                return true;
            }
            return false;
        }
        false
    }

    fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.check_type(token_type) {
            self.advance();
            return true;
        }
        false
    }

    fn parse_expr_primary(&mut self) -> Result<Expr, ParserError> {
        match self.peek().clone() {
            Some(token) => match token.token_type {
                TokenType::STRING
                | TokenType::NUMBER
                | TokenType::TRUE
                | TokenType::FALSE
                | TokenType::NULL => {
                    self.advance();

                    Ok(Expr::Literal { value: token })
                }

                TokenType::LEFT_PAREN => {
                    self.advance();
                    if self.check_type(TokenType::RIGHT_PAREN) {
                        self.advance();
                        self.error_count += 1;
                        return Ok(Expr::Grouping {
                            expr: Box::new(None),
                        });
                    } else {
                        let expr: Result<Expr, ParserError> = self.parse_expr();

                        match self.consume(
                            TokenType::RIGHT_PAREN,
                            "Expected ) after expression".to_string(),
                        ) {
                            Ok(_) => {}
                            Err(e) => return Err(e),
                        }

                        let unwrapped_expr: Expr = match expr {
                            Ok(expr) => expr,
                            Err(e) => return Err(e),
                        };
                        return Ok(Expr::Grouping {
                            expr: Box::new(Some(unwrapped_expr)),
                        });
                    }
                }
                _ => {
                    self.error_count += 1;
                    Err(ParserError::UnexpectedToken(
                        token.line,
                        token.lexeme.to_string(),
                    ))
                }
            },
            None => Err(ParserError::UnexpectedEndOfFile),
        }

        //IMPL ACTUAL PARSER ERROR LATER
    }

    pub fn parse_expr(&mut self) -> Result<Expr, ParserError> {
        //println!("{:?}", self.tokens);
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Result<Expr, ParserError> {
        let mut expr: Expr = self.parse_compare()?;
        while self.match_token(TokenType::BANG_EQUAL) || self.match_token(TokenType::EQUAL_EQUAL) {
            let op: Token = self.view_prev().unwrap();
            let right: Expr = self.parse_compare()?;
            expr = Expr::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_compare(&mut self) -> Result<Expr, ParserError> {
        let mut expr: Expr = self.parse_term()?;

        while self.match_token(TokenType::LESS)
            || self.match_token(TokenType::LESS_EQUAL)
            || self.match_token(TokenType::GREATER)
            || self.match_token(TokenType::GREATER_EQUAL)
        {
            let op: Token = self.view_prev().unwrap();
            let right: Expr = self.parse_term()?;
            expr = Expr::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expr, ParserError> {
        let mut expr: Expr = self.parse_factor()?;

        while self.match_token(TokenType::PLUS) || self.match_token(TokenType::MINUS) {
            let op: Token = self.view_prev().unwrap();
            let right: Expr = self.parse_factor()?;
            expr = Expr::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expr, ParserError> {
        let mut expr: Expr = self.parse_unary()?;
        while self.match_token(TokenType::STAR) || self.match_token(TokenType::SLASH) {
            let op: Token = self.view_prev().unwrap();
            let right: Expr = self.parse_unary()?;
            expr = Expr::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expr, ParserError> {
        if self.match_token(TokenType::BANG) || self.match_token(TokenType::MINUS) {
            let op: Token = self.view_prev().unwrap();
            let right: Expr = self.parse_unary()?;
            return Ok(Expr::Unary {
                op,
                value: Box::new(right),
            });
        }
        return self.parse_expr_primary();
    }
}
