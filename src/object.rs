use std::fmt::Display;
use std::fmt;

#[derive(Debug)]
pub enum Object {
    Boolean {
        value: bool,
    },
    Null,
    Number {
        value: f64,
    },
    String {
        value: String,
    },
}

impl Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Boolean { value } => write!(f, "{}", value),
            Object::Null => write!(f, "null"),
            Object::Number { value } => write!(f, "{}", value),
            Object::String { value } => write!(f, "{}", value),
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Boolean { value: a }, Object::Boolean { value: b }) => a == b,
            (Object::Null, Object::Null) => true,
            (Object::Number { value: a }, Object::Number { value: b }) => a == b,
            (Object::String { value: a }, Object::String { value: b }) => a == b,
            _ => false,
        }
    }
}