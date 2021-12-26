pub mod lexer;
pub mod parser;



#[derive(Debug, Clone)]
enum ExprKind {
    Num,
    Str,
    BinOp,
    CodeBlock,
    Closure,
}
