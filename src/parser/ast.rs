#[derive(Debug, Clone)]
pub enum Expr {
    Num(NumExpr),
    Str(String),
    BinOp(BinOpKind, Box<Expr>, Box<Expr>),
    CodeBlock(Vec<Expr>),
    Closure(Vec<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
    List(Vec<Expr>),
    Null,
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum BinOpKind {
    /// pattern = expr
    Assign,
    /// expr == expr
    Eq,
    /// expr <= expr
    LtEq,
    /// expr >= expr
    GtEq,
    /// expr < expr
    Lt,
    /// expr > expr
    Gt,
    /// expr + expr
    Add,
    /// expr * expr
    Mul,
    /// expr / expr
    Div,
    /// expr % expr
    Mod,
    /// expr - expr
    Sub,
    /// expr ** expr
    Pow,
}

impl Into<u8> for BinOpKind {
    fn into(self) -> u8 {
        match self {
            Self::Assign => 1,
            Self::Eq => 2,
            Self::LtEq => 2,
            Self::GtEq => 2,
            Self::Lt => 2,
            Self::Gt => 2,
            Self::Add => 3,
            Self::Sub => 3,
            Self::Mul => 4,
            Self::Div => 4,
            Self::Mod => 4,
            Self::Pow => 5,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum NumExpr {
    I32(i32),
    I64(i64),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
}
