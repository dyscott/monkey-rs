use std::{collections::HashMap, cell::RefCell, rc::Rc};

#[cfg(test)]
mod tests;

type SymbolScope = &'static str;
type OuterSymbolTable = Rc<RefCell<SymbolTable>>;

pub const GLOBAL_SCOPE: SymbolScope = "GLOBAL";
pub const LOCAL_SCOPE: SymbolScope = "LOCAL";
pub const BUILTIN_SCOPE: SymbolScope = "BUILTIN";
pub const FREE_SCOPE: SymbolScope = "FREE";
pub const FUNCTION_SCOPE: SymbolScope = "FUNCTION";

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
    pub free_symbols: Vec<Symbol>,
}

impl SymbolTable {
    pub fn new(outer: Option<OuterSymbolTable>) -> Rc<RefCell<Self>> {
        let store = HashMap::new();
        Rc::new(RefCell::new(SymbolTable {
            outer,
            store,
            num_definitions: 0,
            free_symbols: vec![],
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

    pub fn define_builtin(&mut self, index: usize, name: &str) -> Symbol {
        let symbol = Symbol {
            name: name.to_string(),
            scope: BUILTIN_SCOPE,
            index,
        };
        self.store.insert(name.to_string(), symbol.clone());
        return symbol;
    }

    pub fn define_free(&mut self, original: &Symbol) -> Symbol {
        self.free_symbols.push(original.clone());
        let symbol = Symbol {
            name: original.name.clone(),
            scope: FREE_SCOPE,
            index: self.free_symbols.len() - 1,
        };
        self.store.insert(original.name.clone(), symbol.clone());
        return symbol;
    }

    pub fn define_function_name(&mut self, name: &str) -> Symbol {
        let symbol = Symbol {
            name: name.to_string(),
            scope: FUNCTION_SCOPE,
            index: 0,
        };
        self.store.insert(name.to_string(), symbol.clone());
        return symbol;
    }

    pub fn resolve(&mut self, name: &str) -> Option<Symbol> {
        let sym = self.store.get(name);

        if let Some(sym) = sym {
            return Some(sym.clone());
        }

        let sym = match &self.outer {
            Some(outer) => match outer.borrow_mut().resolve(name) {
                Some(sym) => sym,
                None => return None,
            },
            None => return None,
        };

        if sym.scope == GLOBAL_SCOPE || sym.scope == BUILTIN_SCOPE {
            return Some(sym);
        }

        let free = self.define_free(&sym);

        return Some(free);
    }
}