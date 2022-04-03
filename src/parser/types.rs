use core::num;
use std::{collections::HashMap, rc::Rc};

use super::{
    ast::{Expr, NumExpr},
    symbols::Symbol,
};

pub type TypeSymbol = Rc<Type>;

#[derive(Debug, Clone)]
pub struct Type {
    name: String,
    kind: TypeKind,
    generics: Vec<TypeSymbol>,
    items: HashMap<String, Symbol>,
}

#[derive(Debug, Clone)]
pub enum TypeKind {
    Struct,
    Enum,
    EnumItem,
    Trait,
    Primitive,
    Unkown,
}

impl From<Expr> for Type {
    fn from(expr: Expr) -> Self {
        match expr {
            Expr::Num(num) => Type {
                name: match num {
                    NumExpr::F32(_) => "f32",
                    NumExpr::F64(_) => "f64",
                    NumExpr::I32(_) => "i32",
                    NumExpr::I64(_) => "i64",
                    NumExpr::U32(_) => "u32",
                    NumExpr::U64(_) => "u64",
                }
                .to_string(),
                generics: Vec::new(),
                items: HashMap::new(),
                kind: TypeKind::Primitive,
            },
            _ => Type {
                name: "Unkown".to_string(),
                generics: Vec::new(),
                items: HashMap::new(),
                kind: TypeKind::Unkown,
            },
        }
    }
}
