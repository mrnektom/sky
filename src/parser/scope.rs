use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use super::symbols::{Symbol, SymbolPtr};
pub type ScopePtr = Rc<Scope>;

#[derive(Debug, Clone)]
pub struct Scope {
    name: String,
    symbols: HashMap<String, SymbolPtr>,
    parent: Option<ScopePtr>,
}

impl Scope {
    pub fn get_sym(&self, name: &str) -> Option<SymbolPtr> {
        Some(self.symbols.get(name)?.to_owned())
    }
    pub fn set_sym(&mut self, name: &str, sym: Symbol) -> Option<SymbolPtr> {
        self.symbols.insert(name.to_string(), Rc::new(sym))
    }
}

pub trait ScopeT {
    fn new_empty() -> Self;
    fn new_named(name: &str) -> Self;
    fn extend(&self) -> Self;
    fn extend_named(&self, name: &str) -> Self;
}

impl ScopeT for ScopePtr {
    fn new_empty() -> Self {
        Rc::new(Scope {
            name: "anonymous".to_string(),
            symbols: HashMap::new(),
            parent: None,
        })
    }
    fn new_named(name: &str) -> Self {
        Rc::new(Scope {
            name: name.to_string(),
            symbols: HashMap::new(),
            parent: None,
        })
    }
    fn extend(&self) -> Self {
        Rc::new(Scope {
            name: "anonymous".to_string(),
            symbols: HashMap::new(),
            parent: Some(self.clone()),
        })
    }
    fn extend_named(&self, name: &str) -> Self {
        Rc::new(Scope {
            name: name.to_string(),
            symbols: HashMap::new(),
            parent: Some(self.clone()),
        })
    }
}
