use std::{
    alloc::{alloc, Layout},
    cell::RefCell,
    collections::HashMap,
    ops::{Deref, DerefMut},
    rc::{Rc, Weak},
};

use super::symbols::{Symbol, SymbolPtr};

#[derive(Debug, Clone)]
pub struct Scope {
    inner: Rc<RefCell<ScopeInner>>,
}

#[derive(Debug, Clone)]
struct ScopeInner {
    name: String,
    symbols: HashMap<String, SymbolPtr>,
    parent: Option<Scope>,
}

impl Scope {
    pub fn new() -> Self {
        Self::new_named("<anonymous>")
    }
    pub fn new_named(name: &str) -> Self {
        let inner = ScopeInner {
            name: name.to_string(),
            symbols: HashMap::new(),
            parent: None,
        };
        Self {
            inner: Rc::new(RefCell::new(inner)),
        }
    }

    pub fn child(&self) -> Self {
        self.child_named("<anonymous>")
    }

    pub fn child_named(&self, name: &str) -> Self {
        let inner = ScopeInner {
            name: name.to_string(),
            symbols: HashMap::new(),
            parent: Some(self.clone()),
        };
        Self {
            inner: Rc::new(RefCell::new(inner)),
        }
    }

    pub fn get_sym(&self, name: &str) -> Option<SymbolPtr> {
        let inner = self.inner.borrow_mut();
        if inner.symbols.contains_key(name) {
            Some(inner.symbols.get(name)?.to_owned())
        } else {
            if inner.parent.is_some() {
                inner.parent.clone().unwrap().get_sym(name)
            } else {
                None
            }
        }
    }
    pub fn set_sym(&mut self, name: &str, sym: Symbol) -> Option<SymbolPtr> {
        let mut inner = self.inner.borrow_mut();
        inner.symbols.insert(name.to_string(), Rc::new(sym))
    }
}
