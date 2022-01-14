mod ast;

pub(crate) mod lexer;

use std::{fmt::Display, slice::SliceIndex};

use self::{
    ast::Expr,
    lexer::{Lexer, LitKind, Token, TokenKind},
};

enum TempVal {
    Expr(Expr),
    Token(Token),
}

impl TempVal {
    pub(self) fn get_tok(self) -> Option<Token> {
        match self {
            Self::Token(tok) => Some(tok),
            _ => None,
        }
    }
    pub(self) fn get_expr(self) -> Option<Expr> {
        match self {
            Self::Expr(expr) => Some(expr),
            _ => None,
        }
    }
    pub(self) fn is_expr(&self) -> bool {
        matches!(self, Self::Expr(_))
    }
    pub(self) fn is_tok(&self) -> bool {
        matches!(self, Self::Token(_))
    }
}

pub struct Parser<'a> {
    code: &'a str,
    lexer: Lexer<'a>,
    pub(self) stack: Vec<TempVal>,
}

impl<'a> Parser<'a> {
    pub fn new(code: &'a str) -> Self {
        Self {
            code,
            lexer: Lexer::new(code),
            stack: Vec::new(),
        }
    }
    pub fn compute_expr(&mut self) {
        while !self.lexer.eof() {
            let tok = self.lexer.peek().unwrap();
            match tok.kind {
                TokenKind::Lit {
                    kind: LitKind::Int { base, suff_off },
                } => {
                    self.stack.push(TempVal::Token(self.lexer.next().unwrap()));
                    self.compute_num();
                }
                _ => {
                    self.print_error();
                    return;
                }
            }
        }
    }
    fn compute_num(&mut self) {
        if self.stack.is_empty() {
            return;
        }
        let tok = self.stack.pop().unwrap();
        if tok.is_expr() {
            return;
        }
        let tok = tok.get_tok().unwrap();
        match tok.kind {
            TokenKind::Lit {
                kind: LitKind::Int { base, suff_off },
            } => {
                let mut start = tok.index;
                let mut end = 0;
                if base.is_some() {
                    start += 2;
                }
                if suff_off.is_none() {
                    end = tok.index + tok.size;
                } else {
                    end = tok.index + suff_off.unwrap();
                }
                let num = self.code.get(start..end);
                let base: u32 = if let Some(base) = base {
                    base.into()
                } else {
                    10
                };
                println!("{}", base);
            }
            _ => (),
        }
    }
    fn can_reduce(&self) {}
    fn reduce(&self) {}
    fn line_by_index(&self, index: usize) -> (String, usize) {
        let mut start = index;
        let mut end = index;
        while let Some(ch) = self.code.get(start..=start) {
            if ch.chars().next().unwrap() == '\n' || start == 0 {
                break;
            }
            start -= 1;
        }
        while let Some(ch) = self.code.get(end..=end) {
            if ch.chars().next().unwrap() == '\n' || end == usize::MAX {
                break;
            }
            end += 1;
        }

        (
            self.code.get(start..end).unwrap().to_string(),
            index - start,
        )
    }
    fn print_error(&mut self) {
        let tok = self.lexer.peek().unwrap().clone();
        let (line, offset) = self.line_by_index(tok.index);
        println!(
            "I don't understand this token \"{}\":\n{}",
            self.code.get(tok.index..tok.index + tok.size).unwrap(),
            line
        );
        let mut buf = String::new();
        buf.push_str(" ".repeat(offset).as_str());
        buf.push_str("^".repeat(tok.size).as_str());
        println!("{}", buf);
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
