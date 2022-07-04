use std::rc::Rc;

pub type TypeSymbol = Rc<Type>;

#[derive(Debug, Clone)]
pub enum Type {
    Struct(StructType),
    Enum,
    EnumVariant,
    Trait,
    Primitive,
    Unkown,
}

#[derive(Debug, Clone)]
pub struct StructType {
    name: String,
    generics: Vec<Generic>,
    fields: Vec<StructField>,
}

#[derive(Debug, Clone)]
pub struct StructField {
    name: String,
    r#type: TypeSymbol,
}
#[derive(Debug, Clone)]
pub struct Generic {
    name: String,
    constraint: Option<TypeSymbol>,
}
