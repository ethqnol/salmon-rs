use crate::ast::stmt::Stmt;
use crate::error::RuntimeError;
use crate::scope::Scope;
use crate::token::Token;
use crate::{interpreter::Interpreter, object::Object};
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct Function {
    name: Token,
    params: Vec<Token>,
    body: Vec<Stmt>,
    closure: Rc<RefCell<Scope>>,
}

impl Function {
    pub fn new(name: Token, params: Vec<Token>, body: Vec<Stmt>, scope : Rc<RefCell<Scope>>) -> Function {
        Function { name, params, body, closure: scope }
    }

    pub fn call(&mut self, interp: &mut Interpreter, args: Vec<Object>) -> Result<Object, RuntimeError> {
        let mut scope = Rc::new(RefCell::new(Scope::from(&self.closure)));
        for i in 0..args.len() {
            scope.borrow_mut().define(format!("arg{}", i), args[i].clone());
        }

        match (*interp).execute_block(&mut self.body, scope) {
            Ok(obj) => obj,
            Err(e) => match e {
                RuntimeError::Return(obj) => return Ok(obj),
                _ => return Err(e),
            },
        };
        
        unimplemented!()
    }

    pub fn arity(&self) -> usize {
        self.params.len()
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<fn {}>", self.name.lexeme)
    }
}
