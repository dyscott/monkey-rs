pub mod environment;
pub mod builtins;

use anyhow::Result;
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{self, Display, Formatter},
    hash::Hash,
    rc::Rc,
};

use crate::{parser::ast::Statement, code::Instructions};
use environment::Environment;

pub type BuiltInFunction = fn(Vec<Object>) -> Result<Object>;

#[derive(Debug, Clone, PartialEq)]
pub struct CompiledFunction {
    pub instructions: Instructions,
    pub num_locals: usize,
    pub num_parameters: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Closure {
    pub func: CompiledFunction,
    pub free: Vec<Object>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    String(String),
    Array(Vec<Object>),
    Hash(HashMap<HashKey, Object>),
    ReturnValue(Box<Object>),
    Function(Vec<String>, Box<Statement>, Rc<RefCell<Environment>>),
    BuiltInFunction(BuiltInFunction),
    CompiledFunction(CompiledFunction),
    Closure(Closure),
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
            Object::Array(values) => {
                let values = values
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "[{}]", values)
            }
            Object::Hash(values) => {
                let values = values
                    .iter()
                    .map(|(key, value)| format!("{}: {}", key, value))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "{{{}}}", values)
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
            Object::CompiledFunction(func) => {
                write!(f, "CompiledFunction[{:p}]", &func)
            }
            Object::Closure(closure) => {
                write!(f, "Closure[{:p}]", &closure)
            }
            Object::Null => {
                write!(f, "null")
            }
        }
    }
}

impl Object {
    // Check if an object is truthy
    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Boolean(value) => *value,
            Object::Null => false,
            _ => true,
        }
    }
    // Get the type name of an object for debugging
    pub fn type_name(&self) -> String {
        match self {
            Object::Integer(_) => "INTEGER",
            Object::Boolean(_) => "BOOLEAN",
            Object::String(_) => "STRING",
            Object::Array(_) => "ARRAY",
            Object::Hash(_) => "HASH",
            Object::ReturnValue(_) => "RETURN_VALUE",
            Object::Function(_, _, _) => "FUNCTION",
            Object::BuiltInFunction(_) => "BUILTIN",
            Object::CompiledFunction(_) => "COMPILED_FUNCTION",
            Object::Closure(_) => "CLOSURE",
            Object::Null => "NULL",
        }
        .to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HashKey {
    Integer(i64),
    Boolean(bool),
    String(String),
}

impl Into<Object> for HashKey {
    // Convert hash keys into objects
    fn into(self) -> Object {
        match self {
            HashKey::Integer(value) => Object::Integer(value),
            HashKey::Boolean(value) => Object::Boolean(value),
            HashKey::String(value) => Object::String(value),
        }
    }
}

impl From<Object> for Option<HashKey> {
    // Convert objects into hash keys
    fn from(value: Object) -> Self {
        match value {
            Object::Integer(value) => Some(HashKey::Integer(value)),
            Object::Boolean(value) => Some(HashKey::Boolean(value)),
            Object::String(value) => Some(HashKey::String(value)),
            _ => None,
        }
    }
}

impl Display for HashKey {
    // Pretty print hash keys
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let object: Object = self.clone().into();
        write!(f, "{}", object)
    }
}
