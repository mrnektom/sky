use std::rc::Rc;

use super::types::TypeSymbol;

pub type SymbolPtr = Rc<Symbol>;

#[derive(Debug, Clone)]
pub enum Symbol {
    Var(VarSymbol),
    Type(TypeSymbol),
    Module,
    Generics(Vec<TypeSymbol>),
    Unkown(UnkownSymbol),
}
#[derive(Debug, Clone)]
pub struct VarSymbol {
    pub name: String,
    pub val_type: TypeSymbol,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone)]
pub struct UnkownSymbol {
    pub name: String,
    pub line: usize,
    pub col: usize,
}
