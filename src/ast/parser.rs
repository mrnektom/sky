use std::fmt::Display;

use crate::ast::Expr;

struct Parser {}

impl Parser {}

#[derive(Debug)]
struct Error {
    text: String,
}
impl Error {
    fn new(msg: String) -> Self {
        Self { text: msg }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.text.as_str())
    }
}

impl std::error::Error for Error {}
