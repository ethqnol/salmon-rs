use std::collections::HashMap;
use crate::{error::RuntimeError, object::Object, token::Token};

#[derive(Debug, Clone)]
pub struct Scope {
    parent: Option<Box<Scope>>,
    values: HashMap<String, Object>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            parent: None,
            values: HashMap::new(),
        }
    }
    
    pub fn define(&mut self, name : String, value : Object) {
        self.values.insert(name, value);
    }
    
    pub fn get(&self, name : &Token) -> Result<Object, RuntimeError> {
        match self.values.get(&name.lexeme) {
            Some(value) => Ok((*value).clone()),
            None => match &self.parent {
                Some(parent) => parent.get(name),
                None => Err(RuntimeError::UndefinedVariable((*name).clone())),
            },
        }
    }
    
    pub fn assign(&mut self, name : &Token, value : Object) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            return Ok(());
        } else {
            match &mut self.parent {
                Some(parent) => parent.assign(name, value),
                None => { return Err(RuntimeError::UndefinedVariable((*name).clone())); }
            }
            
        }
    }
    
    
    
    
}