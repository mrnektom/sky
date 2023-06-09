#[derive(Debug, PartialEq)]
pub struct Module {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Import {
        symbols: Vec<ImportedSymbol>,
        path: String,
    },
    Var {
        name: String,
        is_mut: bool,
        value: Expr,
    },
    Const {
        name: String,
        value: Expr,
    },
    Function {
        name: String,
        params: Vec<FunctionParam>,
        ret_type: TypeUsage,
        body: Vec<Stmt>,
    },
    Expr(Expr),
}

#[derive(Debug, PartialEq)]
pub struct FunctionParam {
    pub name: String,
    pub r#type: TypeUsage,
}

impl FunctionParam {
    pub fn new(name: &str, t: TypeUsage) -> Self {
        Self {
            name: name.to_string(),
            r#type: t,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct TypeUsage {
    pub name: String,
    pub params: Vec<TypeUsage>,
}

impl TypeUsage {
    pub fn from_name(name: &str) -> Self {
        Self {
            name: name.to_string(),
            params: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ImportedSymbol {
    pub name: String,
    pub imported_as: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOpKind {
    /// Addition +
    Add,
    /// Substraction -
    Sub,
    /// Multiply *
    Mul,
    /// Dividion /
    Div,
    /// %
    Rem,
}

impl BinaryOpKind {
    pub fn to_op(&self) -> &str {
        match self {
            BinaryOpKind::Add => "+",
            BinaryOpKind::Sub => "-",
            BinaryOpKind::Mul => "*",
            BinaryOpKind::Div => "/",
            BinaryOpKind::Rem => "%",
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Integer(i32),
    Float(f32),
    String(String),
    Ident(String),
    BinaryOp {
        kind: BinaryOpKind,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Call {
        target: Box<Expr>,
        arguments: Vec<CallArgument>
    },
    DotAccess {
        target: Box<Expr>,
        name: String,
    },
    BracketAccess {
        target: Box<Expr>,
        expr: Box<Expr>
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallArgument {
    pub name: Option<String>,
    pub expr: Expr
}

impl Expr {
    pub fn bin_add(left: Expr, right: Expr) -> Self {
        Self::BinaryOp {
            kind: BinaryOpKind::Add,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn bin_sub(left: Expr, right: Expr) -> Self {
        Self::BinaryOp {
            kind: BinaryOpKind::Sub,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn bin_mul(left: Expr, right: Expr) -> Self {
        Self::BinaryOp {
            kind: BinaryOpKind::Mul,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn bin_div(left: Expr, right: Expr) -> Self {
        Self::BinaryOp {
            kind: BinaryOpKind::Div,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn bin_rem(left: Expr, right: Expr) -> Self {
        Self::BinaryOp {
            kind: BinaryOpKind::Rem,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
    
}

pub mod pattern {
    #[derive(Debug, PartialEq)]
    pub enum Pattern {
        Tuple(Vec<Box<Pattern>>),
        Struct {
            name: String,
            fields: Vec<StructField>,
        },
        Integer(i32),
        Float(f32),
        String(String),
    }

    #[derive(Debug, PartialEq)]
    pub struct StructField {
        pub name: String,
        pub pattern: Pattern,
    }
}
