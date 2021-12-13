pub mod lexer;
pub mod parser;

#[derive(Debug, Clone)]
struct Expr {
    kind: ExprKind,
    tok: Option<String>,
    toks: Option<Vec<Expr>>,
    type_arg: Option<String>,
}

impl Expr {
    pub fn new_num(tok: String) -> Expr {
        Expr {
            kind: ExprKind::Num,
            tok: Some(tok),
            toks: None,
            type_arg: None,
        }
    }

    pub fn new_str(tok: String) -> Expr {
        Expr {
            kind: ExprKind::Str,
            tok: Some(tok),
            toks: None,
            type_arg: None,
        }
    }

    pub fn new_bin_op(tok: String, arg1: Expr, arg2: Expr) -> Expr {
        Expr {
            kind: ExprKind::BinOp,
            tok: Some(tok),
            toks: Some(vec![arg1, arg2]),
            type_arg: None,
        }
    }

    pub fn new_code_block(exprs: Vec<Expr>) -> Self {
        Self {
            kind: ExprKind::CodeBlock,
            tok: None,
            toks: Some(exprs.clone()),
            type_arg: None,
        }
    }

    pub fn kind(self) -> ExprKind {
        self.kind
    }
    pub fn tok(self) -> Option<String> {
        self.tok
    }
    pub fn type_arg(self) -> Option<String> {
        self.type_arg
    }
    pub fn toks(self) -> Option<Vec<Expr>> {
        self.toks
    }
}

#[derive(Debug, Clone)]
enum ExprKind {
    Num,
    Str,
    BinOp,
    CodeBlock,
    Closure,
}
