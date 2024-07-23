use std::f64::consts::PI;

use crate::ast::expr;
use crate::ast::expr::Expr;
use crate::ast::stmt;
use crate::ast::stmt::Stmt;
use crate::error::{ParserError, RuntimeError};
use crate::function::Function;
use crate::object::Object;
use crate::scope::Scope;
use crate::token::{Token, TokenType};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Interpreter {
    pub globals: Rc<RefCell<Scope>>,
    scope: Rc<RefCell<Scope>>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            globals: Rc::new(RefCell::new(Scope::new())),
            scope: Rc::new(RefCell::new(Scope::new())),
        }
    }

    pub fn interpret(&mut self, stmts: &Vec<Stmt>) -> Result<(), RuntimeError> {
        for stmt in stmts {
            self.execute(stmt)?;
        }
        Ok(())
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Result<Object, RuntimeError> {
        return expr.accept(self);
    }

    pub fn check_truthy(&self, object: Object) -> bool {
        match object {
            Object::Boolean { value } => value,
            Object::Null => false,
            _ => return true,
        }
    }

    fn check_equality(&self, left: &Object, right: &Object) -> bool {
        left.eq(right)
    }

    pub fn execute_block(&mut self, stmts: &Vec<Stmt>, scope: Rc<RefCell<Scope>>) -> Result<(), RuntimeError> {
        let previous = self.scope.clone();
        self.scope = scope;
        for stmt in stmts {
            self.execute(stmt)?
        }
        self.scope = previous;
        Ok(())
    }

    pub fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        return stmt.accept(self);
    }

    // fn lookup_variable(&self, name : &Token, expr : Expr) -> Result<Object, RuntimeError> {
    //     let distance = locals.get(name);

    //     if let Some(dist) = distance {
    //         return self.scope.get_at(*dist, name);
    //     } else {
    //         return self.scope.get(name);
    //     }
    // }
}

impl expr::Visitor<Result<Object, RuntimeError>> for Interpreter {
    fn visit_unary(&mut self, op: &Token, value: &Box<Expr>) -> Result<Object, RuntimeError> {
        let right: Object = self.evaluate(value)?;

        match op.token_type {
            TokenType::MINUS => match right {
                Object::Number { value } => return Ok(Object::Number { value: -value }),
                _ => {
                    return Err(RuntimeError::InvalidUnaryOperation(
                        (*op).clone(),
                        "Operator can only be used on numbers".to_string(),
                    ))
                }
            },

            TokenType::BANG => Ok(Object::Boolean {
                value: !self.check_truthy(right),
            }),

            _ => {
                return Err(RuntimeError::InvalidUnaryOperation(
                    (*op).clone(),
                    "".to_string(),
                ))
            }
        }
    }

    fn visit_binary(
        &mut self,
        op: &Token,
        left: &Box<Expr>,
        right: &Box<Expr>,
    ) -> Result<Object, RuntimeError> {
        let right: Object = self.evaluate(right)?;
        let left: Object = self.evaluate(left)?;

        match op.token_type {
            TokenType::AND => {
                if !self.check_truthy(left) {
                    return Ok(Object::Boolean { value: false });
                }
                return Ok(Object::Boolean {
                    value: self.check_truthy(right),
                });
            }

            TokenType::OR => {
                if self.check_truthy(left) {
                    return Ok(Object::Boolean { value: true });
                }
                return Ok(Object::Boolean {
                    value: self.check_truthy(right),
                });
            }

            TokenType::MINUS => match (left, right) {
                (Object::Number { value: l }, Object::Number { value: r }) => {
                    return Ok(Object::Number { value: l - r })
                }
                _ => {
                    return Err(RuntimeError::InvalidBinaryOperation(
                        (*op).clone(),
                        "Operator can only be used on numbers".to_string(),
                    ))
                }
            },

            TokenType::PLUS => match (left, right) {
                (Object::Number { value: l }, Object::Number { value: r }) => {
                    return Ok(Object::Number { value: l + r })
                }
                (Object::String { value: l }, Object::String { value: r }) => {
                    return Ok(Object::String {
                        value: l.clone() + &r,
                    })
                }
                _ => {
                    return Err(RuntimeError::InvalidBinaryOperation(
                        (*op).clone(),
                        "Operator can only be used on numbers or strings".to_string(),
                    ))
                }
            },

            TokenType::GREATER => match (left, right) {
                (Object::Number { value: l }, Object::Number { value: r }) => {
                    return Ok(Object::Boolean { value: l > r })
                }
                _ => {
                    return Err(RuntimeError::InvalidBinaryOperation(
                        (*op).clone(),
                        "Operator can only be used on numbers".to_string(),
                    ))
                }
            },

            TokenType::GREATER_EQUAL => match (left, right) {
                (Object::Number { value: l }, Object::Number { value: r }) => {
                    return Ok(Object::Boolean { value: l >= r })
                }
                _ => {
                    return Err(RuntimeError::InvalidBinaryOperation(
                        (*op).clone(),
                        "Operator can only be used on numbers".to_string(),
                    ))
                }
            },

            TokenType::LESS => match (left, right) {
                (Object::Number { value: l }, Object::Number { value: r }) => {
                    return Ok(Object::Boolean { value: l < r })
                }
                _ => {
                    return Err(RuntimeError::InvalidBinaryOperation(
                        (*op).clone(),
                        "Operator can only be used on numbers".to_string(),
                    ))
                }
            },

            TokenType::LESS_EQUAL => match (left, right) {
                (Object::Number { value: l }, Object::Number { value: r }) => {
                    return Ok(Object::Boolean { value: l <= r })
                }
                _ => {
                    return Err(RuntimeError::InvalidBinaryOperation(
                        (*op).clone(),
                        "Operator can only be used on numbers".to_string(),
                    ))
                }
            },

            TokenType::SLASH => match (left, right) {
                (Object::Number { value: l }, Object::Number { value: r }) => {
                    return Ok(Object::Number { value: l / r })
                }
                _ => {
                    return Err(RuntimeError::InvalidBinaryOperation(
                        (*op).clone(),
                        "Operator can only be used on numbers".to_string(),
                    ))
                }
            },

            TokenType::STAR => match (left, right) {
                (Object::Number { value: l }, Object::Number { value: r }) => {
                    return Ok(Object::Number { value: l * r })
                }
                _ => {
                    return Err(RuntimeError::InvalidBinaryOperation(
                        (*op).clone(),
                        "Operator can only be used on numbers".to_string(),
                    ))
                }
            },

            TokenType::EQUAL_EQUAL => {
                return Ok(Object::Boolean {
                    value: self.check_equality(&left, &right),
                });
            }

            TokenType::BANG_EQUAL => {
                return Ok(Object::Boolean {
                    value: !self.check_equality(&left, &right),
                });
            }

            _ => {
                return Err(RuntimeError::InvalidBinaryOperation(
                    (*op).clone(),
                    "".to_string(),
                ))
            }
        }
    }

    fn visit_literal(&mut self, value: &Token) -> Result<Object, RuntimeError> {
        match value.token_type {
            TokenType::NUMBER => {
                return Ok(Object::Number {
                    value: value.lexeme.parse::<f64>().unwrap(),
                })
            }
            TokenType::STRING => {
                return Ok(Object::String {
                    value: value.lexeme.clone(),
                })
            }
            TokenType::TRUE => return Ok(Object::Boolean { value: true }),
            TokenType::FALSE => return Ok(Object::Boolean { value: false }),
            TokenType::NULL => return Ok(Object::Null),
            _ => {
                return Err(RuntimeError::InvalidLiteral(
                    (*value).clone(),
                    "".to_string(),
                ))
            }
        }
    }

    fn visit_grouping(&mut self, expr: &Box<Expr>) -> Result<Object, RuntimeError> {
        return self.evaluate(expr);
    }

    fn visit_logical(
        &mut self,
        op: &Token,
        left: &Box<Expr>,
        right: &Box<Expr>,
    ) -> Result<Object, RuntimeError> {
        let left: Object = self.evaluate(left)?;

        match op.token_type {
            TokenType::AND => {
                if !self.check_truthy(left) {
                    return Ok(Object::Boolean { value: false });
                }
                return self.evaluate(right);
            }

            TokenType::OR => {
                if self.check_truthy(left) {
                    return Ok(Object::Boolean { value: true });
                }
                return self.evaluate(right);
            }

            _ => {
                return Err(RuntimeError::InvalidLogicalOperation(
                    (*op).clone(),
                    "".to_string(),
                ))
            }
        }
    }

    fn visit_variable(&mut self, name: &Token) -> Result<Object, RuntimeError> {
        self.scope.borrow().get(name)
    }

    fn visit_assign(&mut self, name: &Token, value: &Box<Expr>) -> Result<Object, RuntimeError> {
        let value = self.evaluate(value)?;
        self.scope.borrow_mut().assign(name, value.clone());
        return Ok(value);
    }

    fn visit_call(
        &mut self,
        callee: &Box<Expr>,
        paren: &Token,
        arguments: &Vec<Expr>,
    ) -> Result<Object, RuntimeError> {
        let callee: Object = self.evaluate(callee)?;
        let mut args: Vec<Object> = Vec::new();
        for arg in arguments {
            args.push(self.evaluate(arg)?);
        }

        match callee {
            Object::Callable { func: mut f } => {
                if args.len() != f.arity() {
                    return Err(RuntimeError::InvalidFunctionCall(
                        (*paren).clone(),
                        "Incorrect number of arguments".to_string(),
                    ));
                } else {
                    return f.call(self, args);
                }
            }
            _ => {
                return Err(RuntimeError::InvalidFunctionCall(
                    (*paren).clone(),
                    "Can only call functions".to_string(),
                ))
            }
        };
    }
}

impl stmt::Visitor<Result<(), RuntimeError>> for Interpreter {
    fn visit_var(&mut self, name: &Token, initializer: &Option<Expr>) -> Result<(), RuntimeError> {
        let value: Object = match initializer {
            Some(expr) => self.evaluate(expr)?,
            None => Object::Null,
        };
        self.scope.borrow_mut().define(name.clone().lexeme, value);
        Ok(())
    }

    fn visit_block(&mut self, statements: &Vec<Stmt>) -> Result<(), RuntimeError> {
        self.execute_block(statements, Rc::new(RefCell::new(Scope::from(&self.scope))));
        Ok(())
    }

    fn visit_expression(&mut self, expr: &Expr) -> Result<(), RuntimeError> {
        self.evaluate(expr)?;
        Ok(())
    }

    fn visit_if(
        &mut self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: &Option<Box<Stmt>>,
    ) -> Result<(), RuntimeError> {
        let cond = self.evaluate(condition)?;
        if self.check_truthy(cond) {
            self.execute(then_branch)?;
        } else if let Some(else_branch) = else_branch {
            self.execute(else_branch)?;
        }

        Ok(())
    }

    fn visit_class(&mut self, name: &Token, methods: &Vec<Stmt>) -> Result<(), RuntimeError> {
        unimplemented!()
    }

    fn visit_print(&mut self, expr: &Expr) -> Result<(), RuntimeError> {
        println!("{}", self.evaluate(expr)?);
        Ok(())
    }

    fn visit_while(&mut self, condition: &Expr, body: &Stmt) -> Result<(), RuntimeError> {
        let mut value = self.evaluate(condition)?;
        while self.check_truthy(value) {
            self.execute(body)?;
            value = self.evaluate(condition)?;
        }

        Ok(())
    }

    fn visit_return(&mut self, keyword: &Token, value: &Option<Expr>) -> Result<(), RuntimeError> {
        let val: Object = match value {
            Some(expr) => self.evaluate(expr)?,
            None => Object::Null,
        };

        Err(RuntimeError::Return(val))
    }

    fn visit_function(
        &mut self,
        name: &Token,
        params: &Vec<Token>,
        body: &Vec<Stmt>,
    ) -> Result<(), RuntimeError> {
        let func = Object::Callable {
            func: Function::new(
                name.clone(),
                params.clone(),
                body.clone(),
                Rc::clone(&self.scope),
            ),
        };
        self.scope.borrow_mut().define(name.lexeme.clone(), func);
        Ok(())
    }
}
