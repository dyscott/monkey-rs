use std::collections::HashMap;

#[cfg(test)]
mod tests;

type SymbolScope = &'static str;

const GLOBAL_SCOPE: SymbolScope = "GLOBAL";

#[derive(Debug, PartialEq, Clone)]
pub struct Symbol {
    pub name: String,
    pub scope: SymbolScope,
    pub index: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SymbolTable {
    pub store: HashMap<String, Symbol>,
    pub num_definitions: usize,
}

impl SymbolTable {
    pub fn new() -> Self {
        let store = HashMap::new();
        SymbolTable {
            store,
            num_definitions: 0,
        }
    }

    pub fn define(&mut self, name: &str) -> Symbol {
        let symbol = Symbol {
            name: name.to_string(),
            scope: GLOBAL_SCOPE,
            index: self.num_definitions,
        };
        self.store.insert(name.to_string(), symbol.clone());
        self.num_definitions += 1;
        return symbol;
    }

    pub fn resolve(&self, name: &str) -> Option<Symbol> {
        self.store.get(name).cloned()
    }
}