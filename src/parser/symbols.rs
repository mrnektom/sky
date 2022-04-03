use std::rc::Rc;

use super::types::TypeSymbol;

pub type SymbolPtr = Rc<Symbol>;
#[derive(Debug, Clone)]
pub enum Symbol {
    Var(TypeSymbol),
    Type,
    Module,
    Unkown,
    Field,
}

struct VarSymbol {
    name: String,
    val_type: TypeSymbol,
}
