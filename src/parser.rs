use crate::ast::expr::Expr;
use crate::ast::stmt::Stmt;
use crate::error::ParserError;
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

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParserError> {
        let mut stmts: Vec<Stmt> = Vec::new();
        while !self.is_end() {
            let stmt = self.declaration();
            match stmt {
                Ok(stmt) => stmts.push(stmt),
                Err(e) => {
                    self.error_count += 1;
                    eprintln!("{}", e);
                }
            }
        }
        Ok(stmts)
    }

    fn declaration(&mut self) -> Result<Stmt, ParserError> {
        let stmt = if self.match_token(TokenType::VAR) {
            self.var_declaration()
        } else if self.match_token(TokenType::CLASS) {
            self.class_declaration()
        } else if self.match_token(TokenType::FN) {
            self.function_declaration()
        } else {
            self.statement()
        };

        match stmt {
            Ok(stmt) => Ok(stmt),
            Err(e) => {
                self.error_count += 1;
                Err(e)
            }
        }
    }
    fn statement(&mut self) -> Result<Stmt, ParserError> {
        if self.match_token(TokenType::PRINT) {
            self.print_statement()
        } else if self.match_token(TokenType::IF) {
            self.if_statement()
        } else if self.match_token(TokenType::FOR) {
            self.for_statement()
        } else if self.match_token(TokenType::WHILE) {
            self.while_statement()
        } else if self.match_token(TokenType::RETURN){
            self.return_statement()
        } else if self.match_token(TokenType::LEFT_BRACE) {
            Ok(Stmt::Block {
                stmts: self.block()?,
            })
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Result<Stmt, ParserError> {
        self.consume(
            TokenType::LEFT_PAREN,
            "Expect '(' after 'print'.".to_string(),
        )?;
        let expr = self.parse_expr()?;
        self.consume(
            TokenType::RIGHT_PAREN,
            "Expect ')' after value.".to_string(),
        )?;
        self.consume(TokenType::SEMICOLON, "Expect ';' after value.".to_string())?;
        Ok(Stmt::Print {
            expr,
        })
    }

    fn if_statement(&mut self) -> Result<Stmt, ParserError> {
        self.consume(TokenType::LEFT_PAREN, "Expect '(' after 'if'.".to_string())?;
        let condition = self.parse_expr()?;
        self.consume(
            TokenType::RIGHT_PAREN,
            "Expect ')' after if condition.".to_string(),
        )?;

        let then_branch = Box::new(self.statement()?);
        let mut else_branch: Option<Box<Stmt>> = None;
        if self.match_token(TokenType::ELSE) {
            else_branch = Some(Box::new(self.statement()?));
        }

        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn while_statement(&mut self) -> Result<Stmt, ParserError> {
        self.consume(
            TokenType::LEFT_PAREN,
            "Expect '(' after 'while'.".to_string(),
        )?;
        let condition = self.parse_expr()?;
        self.consume(
            TokenType::RIGHT_PAREN,
            "Expect ')' after while condition.".to_string(),
        )?;

        let body = Box::new(self.statement()?);

        Ok(Stmt::While {
            condition,
            body,
        })
    }
    
    fn return_statement(&mut self) -> Result<Stmt, ParserError> {
        let keyword = self.view_prev().unwrap().clone();
        let value = if !self.check_type(TokenType::SEMICOLON) {
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.consume(TokenType::SEMICOLON, "Expect ';' after return value.".to_string())?;
        Ok(Stmt::Return {
            keyword,
            value,
        })
    }

    fn for_statement(&mut self) -> Result<Stmt, ParserError> {
        self.consume(TokenType::LEFT_PAREN, "Expect '(' after 'for'.".to_string())?;

        let initializer = if self.match_token(TokenType::SEMICOLON) {
            None
        } else if self.match_token(TokenType::VAR) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if !self.check_type(TokenType::SEMICOLON) {
            Some(self.parse_expr()?)
        } else {
            None
        };

        self.consume(
            TokenType::SEMICOLON,
            "Expect ';' after loop condition.".to_string(),
        )?;

        let increment = if !self.check_type(TokenType::RIGHT_PAREN) {
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.consume(
            TokenType::RIGHT_PAREN,
            "Expect ')' after for clauses.".to_string(),
        )?;

        let mut body = self.statement()?;

        if let Some(increment) = increment {
            body = Stmt::Block {
                stmts: vec![
                    body,
                    Stmt::Expression {
                        expr: increment,
                    },
                ],
            };
        }

        body = Stmt::While {
            condition: condition.unwrap_or(Expr::Literal {
                value: Token::new(
                    TokenType::TRUE,
                    "true".to_string(),
                    self.view_prev().clone().unwrap().line,
                ),
            }),
            body: Box::new(body),
        };

        Ok(body)
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParserError> {
        let expr = self.parse_expr()?;
        self.consume(
            TokenType::SEMICOLON,
            "Expect ';' after expression.".to_string(),
        )?;
        Ok(Stmt::Expression {
            expr,
        })
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParserError> {
        let name = match self.consume(TokenType::IDENTIFIER, "Expect variable name.".to_string()) {
            Ok(token) => token,
            Err(e) => return Err(e),
        };

        let mut initializer: Option<Expr> = None;
        if self.match_token(TokenType::EQUAL) {
            initializer = Some(self.parse_expr()?);
        } else {
            initializer = None;
        }
        self.consume(
            TokenType::SEMICOLON,
            "Expect ';' after variable declaration.".to_string(),
        )?;

        Ok(Stmt::Var { name, initializer })
    }

    fn class_declaration(&mut self) -> Result<Stmt, ParserError> {
        let name = match self.consume(TokenType::IDENTIFIER, "Expect class name.".to_string()) {
            Ok(token) => token,
            Err(e) => return Err(e),
        };
        let mut methods: Vec<Stmt> = Vec::new();
        while !self.check_type(TokenType::EOF) && !self.check_type(TokenType::RIGHT_BRACE) {
            let method = self.function_declaration();
            match method {
                Ok(method) => methods.push(method),
                Err(e) => {
                    self.error_count += 1;
                    eprintln!("{}", e);
                }
            }
        }
        self.consume(
            TokenType::RIGHT_BRACE,
            "Expected } after class declaration".to_string(),
        )?;
        Ok(Stmt::Class { name, methods })
    }

    fn function_declaration(&mut self) -> Result<Stmt, ParserError> {
        let name = self.consume(TokenType::IDENTIFIER, "Expect function name.".to_string())?;
        self.consume(
            TokenType::LEFT_PAREN,
            "Expect '(' after function name.".to_string(),
        )?;

        let mut params: Vec<Token> = Vec::new();
        if !self.check_type(TokenType::RIGHT_PAREN) {
            loop {
                if params.len() >= 255 {
                    return Err(ParserError::InvalidExpression(
                        self.peek().unwrap().line,
                        "Cannot have more than 255 parameters.".to_string(),
                    ));
                }
                let param =
                    self.consume(TokenType::IDENTIFIER, "Expect parameter name.".to_string())?;
                params.push(param);
                if !self.match_token(TokenType::COMMA) {
                    break;
                }
            }
        }

        self.consume(
            TokenType::RIGHT_PAREN,
            "Expect ')' after parameters.".to_string(),
        )?;
        self.consume(
            TokenType::LEFT_BRACE,
            "Expect '{' before function body.".to_string(),
        )?;

        let mut body = self.block()?;
        Ok(Stmt::Function { name, params, body })
    }

    fn block(&mut self) -> Result<Vec<Stmt>, ParserError> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.check_type(TokenType::RIGHT_BRACE) && !self.is_end() {
            statements.push(self.declaration()?);
        }

        self.consume(
            TokenType::RIGHT_BRACE,
            "Expect '}' after block.".to_string(),
        )?;
        Ok(statements)
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
                        return Err(ParserError::InvalidExpression(
                            token.line,
                            "Expected expression inside parentheses".to_string(),
                        ));
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
                            expr: Box::new(unwrapped_expr),
                        });
                    }
                }
                TokenType::IDENTIFIER => {
                    let token = self.advance().unwrap();
                    Ok(Expr::Variable { name: token })
                }
                _ => {
                    self.error_count += 1;
                    self.advance();
                    Err(ParserError::UnexpectedToken(
                        token.line,
                        token.lexeme.to_string(),
                    ))
                }
            },
            None => Err(ParserError::UnexpectedEndOfFile),
        }
    }

    pub fn parse_expr(&mut self) -> Result<Expr, ParserError> {
        self.parse_assign()
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

    fn parse_assign(&mut self) -> Result<Expr, ParserError> {
        let expr: Expr = self.parse_or()?;
        if self.match_token(TokenType::EQUAL) {
            let eq: Token = self.view_prev().clone().unwrap();
            let value: Expr = self.parse_assign()?;

            if let Expr::Variable { name } = expr {
                return Ok(Expr::Assign {
                    name,
                    value: Box::new(value),
                });
            } else {
                self.error_count += 1;
                return Err(ParserError::UnexpectedToken(
                    eq.line,
                    "Expected variable name".to_string(),
                ));
            }
        }
        Ok(expr)
    }

    fn parse_or(&mut self) -> Result<Expr, ParserError> {
        let mut expr: Expr = self.parse_and()?;
        while self.match_token(TokenType::OR) {
            let op: Token = self.view_prev().unwrap();
            let right: Expr = self.parse_and()?;
            expr = Expr::Logical {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_and(&mut self) -> Result<Expr, ParserError> {
        let mut expr: Expr = self.parse_equality()?;
        while self.match_token(TokenType::AND) {
            let op: Token = self.view_prev().unwrap();
            let right: Expr = self.parse_equality()?;
            expr = Expr::Logical {
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
        return self.parse_call();
    }

    fn parse_call(&mut self) -> Result<Expr, ParserError> {
        let mut expr: Expr = self.parse_expr_primary()?;
        loop {
            if self.match_token(TokenType::LEFT_PAREN) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn finish_call(&mut self, expr: Expr) -> Result<Expr, ParserError> {
        let mut arguments: Vec<Expr> = Vec::new();
        if !self.check_type(TokenType::RIGHT_PAREN) {
            loop {
                if arguments.len() >= 255 {
                    self.error_count += 1;
                    return Err(ParserError::FunctionError(
                        self.view_prev().unwrap().line,
                        "Too many arguments".to_string(),
                    ));
                }
                arguments.push(self.parse_expr()?);
                if !self.match_token(TokenType::COMMA) {
                    break;
                }
            }
        }
        let paren = self.consume(
            TokenType::RIGHT_PAREN,
            "Expected ')' after arguments".to_string(),
        )?;
        return Ok(Expr::Call {
            callee: Box::new(expr),
            paren,
            arguments,
        });
    }
}
