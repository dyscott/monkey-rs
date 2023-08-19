use std::{collections::HashMap, cell::RefCell, rc::Rc};

use super::object::Object;

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    values: HashMap<String, Object>,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Default for Environment {
    // Create a new environment with no outer environment
    fn default() -> Self {
        Self::new(None)
    }
}

impl Environment {
    // Create a new environment with an optional outer environment
    pub fn new(outer: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            values: HashMap::new(),
            outer,
        }
    }

    // Get a value from the environment
    pub fn get(&self, name: &String) -> Option<Object> {
        match self.values.get(name) {
            Some(value) => Some(value.clone()),
            None => match &self.outer {
                Some(outer) => outer.borrow().get(name),
                None => None,
            },
        }
    }

    // Set a value in the environment
    pub fn set(&mut self, name: &String, value: Object) {
        self.values.insert(name.to_string(), value);
    }
}