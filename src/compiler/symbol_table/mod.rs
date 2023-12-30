use std::{collections::HashMap, cell::RefCell, rc::Rc};

#[cfg(test)]
mod tests;

type SymbolScope = &'static str;
type OuterSymbolTable = Rc<RefCell<SymbolTable>>;

pub const GLOBAL_SCOPE: SymbolScope = "GLOBAL";
pub const LOCAL_SCOPE: SymbolScope = "LOCAL";

#[derive(Debug, PartialEq, Clone)]
pub struct Symbol {
    pub name: String,
    pub scope: SymbolScope,
    pub index: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SymbolTable {
    pub outer: Option<Rc<RefCell<SymbolTable>>>,
    pub store: HashMap<String, Symbol>,
    pub num_definitions: usize,
}

impl SymbolTable {
    pub fn new(outer: Option<OuterSymbolTable>) -> Rc<RefCell<Self>> {
        let store = HashMap::new();
        Rc::new(RefCell::new(SymbolTable {
            outer,
            store,
            num_definitions: 0,
        }))
    }

    pub fn define(&mut self, name: &str) -> Symbol {
        let symbol = Symbol {
            name: name.to_string(),
            scope: match self.outer {
                Some(_) => LOCAL_SCOPE,
                None => GLOBAL_SCOPE,
            },
            index: self.num_definitions,
        };
        self.store.insert(name.to_string(), symbol.clone());
        self.num_definitions += 1;
        return symbol;
    }

    pub fn resolve(&self, name: &str) -> Option<Symbol> {
        match self.store.get(name) {
            Some(symbol) => Some(symbol.clone()),
            None => match &self.outer {
                Some(outer) => outer.borrow().resolve(name),
                None => None,
            },
        }
    }
}