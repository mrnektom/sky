mod ast;
pub(crate) mod lexer;

use std::{collections::HashMap, fmt::Display, fs::File, io::Read};

use self::{
    ast::Expr,
    lexer::{Lexer, TokenKind},
};

pub struct Parser {
    lexer: Lexer,
    pub(self) stack: Vec<Expr>,
}

impl Parser {
    pub fn from_path(code: &'static str) -> Result<Parser, Box<dyn std::error::Error>> {
        Ok(Self {
            lexer: Lexer::new(code),
            stack: Vec::new(),
        })
    }
}

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
