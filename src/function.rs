use std::fmt;
use crate::{interpreter::Interpreter, object::Object};

#[derive(Debug, Clone)]
pub struct Function {
    arity : usize
}

impl Function {
    pub fn new() -> Function {
        Function {
            arity : 0
        }
    }
    
    pub fn call(&self, interp : &Interpreter, args : Vec<Object>) -> Object {
        unimplemented!();
    }
    
    pub fn arity(&self) -> usize {
        self.arity
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Function")
    }
}