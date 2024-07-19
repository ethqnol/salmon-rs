use crate::object::Object;
use crate::error::RuntimeError;
use crate::ast::{expr::Expr, expr::Visitor};
use crate::token::{ TokenType, Token };

pub struct Interpreter {
    
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            
        }
    }
    
    pub fn interpret(&self, expr : Expr) -> Result<Object, RuntimeError> {
        let res = self.evaluate(expr);
        
        match res {
            Ok(object) => {
                println!("{}", object);
                return Ok(object);
            }
            Err(e) => {
                println!("{}", e);
                return Err(e);
            }
        }
    }
    
    pub fn evaluate(&self, expr : Expr) -> Result<Object, RuntimeError> {
        return expr.accept(self);
    }
    
    pub fn check_truthy(&self, object : Object) -> bool {
        match object {
            Object::Boolean{ value } => value,
            Object::Null => false,
            _ => return true,
        }
    }
    
    fn check_equality(&self, left : &Object, right : &Object) -> bool {
        left.eq(right)
    }
    
}

impl Visitor<Result<Object, RuntimeError>> for Interpreter {
    fn visit_unary(&self, op: &Token, value: &Box<Expr>) -> Result<Object, RuntimeError> {
        let right : Object = self.evaluate(*value.clone())?;
        
        match op.token_type {
            TokenType::MINUS => {
                match right {
                    Object::Number{ value } => return Ok(Object::Number{ value: -value }),
                    _ => return Err(RuntimeError::new((*op).clone(), "Unary operator - can only be applied to numbers".to_string())),
                }
            }
            
            TokenType::BANG=>  Ok(Object::Boolean{ value: !self.check_truthy(right)}) ,
            
            _ => return Err(RuntimeError::new((*op).clone(), "Invalid unary operator".to_string())),
        }
    }

    fn visit_binary(&self, op: &Token, left: &Box<Expr>, right: &Box<Expr>) -> Result<Object, RuntimeError> {
        let right : Object = self.evaluate(*right.clone())?;
        let left : Object = self.evaluate(*left.clone())?;
        
        match op.token_type {
            TokenType::MINUS => {
                match  (left, right) {
                    (Object::Number{ value: l }, Object::Number{ value: r }) => return Ok(Object::Number{ value: l - r }),
                    _ => return Err(RuntimeError::new((*op).clone(), "Binary operator - can only be applied to numbers".to_string())),
                }
            } 
            
            TokenType::PLUS => {
                match(left, right) {
                    (Object::Number{ value: l }, Object::Number{ value: r }) => return Ok(Object::Number{ value: l + r }),
                    (Object::String{ value: l }, Object::String{ value: r }) => return Ok(Object::String{ value: l.clone() + &r }),
                    _ => return Err(RuntimeError::new((*op).clone(), "Binary operator + can only be applied to numbers or strings".to_string())),
                }
            }
            
            TokenType::GREATER => {
                match(left, right) {
                    (Object::Number{ value: l }, Object::Number{ value: r }) => return Ok(Object::Boolean{ value: l > r }),
                    _ => return Err(RuntimeError::new((*op).clone(), "Binary operator > can only be applied to numbers".to_string())),
                }
            }
            
            TokenType::GREATER_EQUAL => {
                match(left, right) {
                    (Object::Number{ value: l }, Object::Number{ value: r }) => return Ok(Object::Boolean{ value: l >= r }),
                    _ => return Err(RuntimeError::new((*op).clone(), "Binary operator >= can only be applied to numbers".to_string())),
                }
            }
            
            TokenType::LESS => {
                match(left, right) {
                    (Object::Number{ value: l }, Object::Number{ value: r }) => return Ok(Object::Boolean{ value: l < r }),
                    _ => return Err(RuntimeError::new((*op).clone(), "Binary operator < can only be applied to numbers".to_string())),
                }
            }
            
            TokenType::LESS_EQUAL => {
                match(left, right) {
                    (Object::Number{ value: l }, Object::Number{ value: r }) => return Ok(Object::Boolean{ value: l <= r }),
                    _ => return Err(RuntimeError::new((*op).clone(), "Binary operator <= can only be applied to numbers".to_string())),
                }
            }
            
            TokenType::SLASH => {
                match(left, right) {
                    (Object::Number{ value: l }, Object::Number{ value: r }) => return Ok(Object::Number{ value: l / r }),
                    _ => return Err(RuntimeError::new((*op).clone(), "Binary operator / can only be applied to numbers".to_string())),
                }
            }
            
            TokenType::STAR => {
                match(left, right) {
                    (Object::Number{ value: l }, Object::Number{ value: r }) => return Ok(Object::Number{ value: l * r }),
                    _ => return Err(RuntimeError::new((*op).clone(), "Binary operator * can only be applied to numbers".to_string())),
                }
            }
            
            TokenType::EQUAL_EQUAL => {
                return Ok(Object::Boolean{ value: self.check_equality(&left, &right)});
            }
            
            TokenType::BANG_EQUAL => {
                return Ok(Object::Boolean{ value: !self.check_equality(&left, &right) });
            }
            
            _=> return Err(RuntimeError::new((*op).clone(), "Invalid binary operator".to_string())),
        }
    }

    fn visit_literal(&self, value: &Token) -> Result<Object, RuntimeError> {
        match value.token_type {
            TokenType::NUMBER => return Ok(Object::Number{ value: value.lexeme.parse::<f64>().unwrap() }),
            TokenType::STRING => return Ok(Object::String{ value: value.lexeme.clone() }),
            TokenType::TRUE => return Ok(Object::Boolean{ value: true }),
            TokenType::FALSE => return Ok(Object::Boolean{ value: false }),
            TokenType::NULL => return Ok(Object::Null),
            _ => return Err(RuntimeError::new((*value).clone(), "Invalid literal".to_string())),
        }
    }

    fn visit_grouping(&self, expr: &Box<Option<Expr>>) -> Result<Object, RuntimeError> {
        return self.evaluate((*expr.clone()).unwrap())
    }
}