use std::{fmt::{self, Display, Formatter}, cell::RefCell, rc::Rc};

use crate::parser::ast::Statement;

use super::environment::Environment;
use super::builtins::BuiltInFunction;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    String(String),
    ReturnValue(Box<Object>),
    Function(Vec<String>, Box<Statement>, Rc<RefCell<Environment>>),
    BuiltInFunction(BuiltInFunction),
    Null,
}

impl Display for Object {
    // Pretty print objects
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Object::Integer(value) => {
                write!(f, "{}", value)
            }
            Object::Boolean(value) => {
                write!(f, "{}", value)
            }
            Object::String(value) => {
                write!(f, "{}", value)
            }
            Object::ReturnValue(value) => {
                write!(f, "{}", value)
            }
            Object::Function(params, body, _) => {
                let params = params.join(", ");
                write!(f, "fn({}) {{\n{}\n}}", params, body)
            }
            Object::BuiltInFunction(_) => {
                write!(f, "builtin function")
            }
            Object::Null => {
                write!(f, "null")
            }
        }
    }
}

impl Object {
    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Boolean(value) => *value,
            Object::Null => false,
            _ => true,
        }
    }
    pub fn type_name(&self) -> String {
        match self {
            Object::Integer(_) => "INTEGER",
            Object::Boolean(_) => "BOOLEAN",
            Object::String(_) => "STRING",
            Object::ReturnValue(_) => "RETURN_VALUE",
            Object::Function(_, _, _) => "FUNCTION_OBJ",
            Object::BuiltInFunction(_) => "BUILTIN",
            Object::Null => "NULL",
        }
        .to_string()
    }
}
