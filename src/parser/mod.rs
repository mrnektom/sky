pub mod ast;

pub(crate) mod lexer;

use std::usize;

use self::{
    ast::{BinOpKind, Expr, NumExpr},
    lexer::{Lexer, LitKind, Token, TokenKind},
};
#[derive(Debug)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    stack: Vec<Expr>,
    code: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(code: &'a str) -> Self {
        Self {
            lexer: Lexer::new(code),
            stack: Vec::new(),
            code,
        }
    }
    pub fn parse_top(&mut self) -> Expr {
        let mut exprs = Vec::new();
        while !self.lexer.eof() {
            self.parse_expr();
            if !self.stack.is_empty() {
                exprs.push(self.stack.pop().unwrap());
            } else {
                break;
            }
        }
        if exprs.len() == 1 {
            exprs.pop().unwrap()
        } else {
            Expr::CodeBlock(exprs)
        }
    }
    fn parse_expr(&mut self) {
        self.parse_atom();
        self.maybe_call();
        self.maybe_binary();
    }
    fn parse_atom(&mut self) {
        self.skip_whitespace();
        let tok = self.lexer.peek();
        if tok.is_none() {
            return;
        }
        let tok = tok.unwrap().to_owned();
        match tok.kind {
            TokenKind::Lit { kind } => match kind {
                LitKind::Num { .. } => self.parse_num(),
                LitKind::Str => self.parse_str(),
            },
            _ => self.print_error("Invalid token recivied", tok.index, tok.size),
        }
    }
    fn parse_num(&mut self) {
        if let Some(Token {
            kind:
                TokenKind::Lit {
                    kind: LitKind::Num { base, suff_off },
                },
            size,
            index,
        }) = self.lexer.next()
        {
            let mut start = index;
            let mut end = index + size;
            let mut radix = 10;
            if let Some(base) = base {
                start += 2;
                radix = base.into();
            }
            let suff = match suff_off {
                Some(offset) => {
                    end = index + offset;
                    self.code.get(index + offset..index + size)
                }
                None => None,
            };
            let mut val = self.code.get(start..end).unwrap();

            let expr = Expr::Num(match suff {
                Some("i32") => {
                    if val.contains('.') {
                        val = val.get(..val.find('.').unwrap()).unwrap();
                    }
                    NumExpr::I32(i32::from_str_radix(val, radix).unwrap())
                }
                Some("i64") => {
                    if val.contains('.') {
                        val = val.get(..val.find('.').unwrap()).unwrap();
                    }
                    NumExpr::I64(i64::from_str_radix(val, radix).unwrap())
                }
                Some("u32") => {
                    if val.contains('.') {
                        val = val.get(..val.find('.').unwrap()).unwrap();
                    }
                    NumExpr::U32(u32::from_str_radix(val, radix).unwrap())
                }
                Some("u64") => {
                    if val.contains('.') {
                        val = val.get(..val.find('.').unwrap()).unwrap();
                    }
                    NumExpr::U64(u64::from_str_radix(val, radix).unwrap())
                }
                Some("f32") => NumExpr::F32(parse_based_f32(base.unwrap().into(), val).unwrap()),
                Some("f64") => NumExpr::F64(parse_based_f64(base.unwrap().into(), val).unwrap()),
                None if val.contains('.') => NumExpr::F32(val.parse().unwrap()),
                None => NumExpr::I32(i32::from_str_radix(val, radix).unwrap()),
                Some(suff) => {
                    self.print_error(
                        format!("Invalid number suffix recivied \"{}\"", suff).as_str(),
                        index + suff_off.unwrap(),
                        suff.len(),
                    );
                    return;
                }
            });
            self.stack.push(expr);
        }
    }

    fn parse_str(&mut self) {
        if let Some(Token {
            kind: _,
            size,
            index,
        }) = self.lexer.next()
        {
            let string = self.code.get(index + 1..index + size - 1).unwrap();

            self.stack.push(Expr::Str(escape_str(string)));
        }
    }

    fn maybe_call(&self) {}
    fn maybe_binary(&mut self) {
        self.skip_whitespace();
        if self.stack.is_empty() {
            return;
        }
        if self.lexer.eof() {
            return;
        }
        let Token {
            kind: _,
            size: _,
            index,
        } = self.lexer.peek().unwrap();
        if let Some(kind) = self.parse_bin_op() {
            let priory: u8 = kind.clone().into();
            let last = self.stack.pop();
            if last.is_none() {
                let Token {
                    kind: _,
                    size: _,
                    index: i,
                } = self.lexer.peek().unwrap();
                self.print_error("Unexpected token", index, i - index);
                return;
            }
            let last = last.unwrap();
            let mut expr: Expr;
            self.parse_expr();
            let right = self.stack.pop().unwrap();
            if let Expr::BinOp(k, l, r) = right {
                let r_priory: u8 = k.clone().into();
                if priory >= r_priory {
                    println!(">");
                    expr = Expr::BinOp(kind, Box::new(last), l);
                    expr = Expr::BinOp(k, Box::new(expr), r);
                } else {
                    println!("<");
                    expr = Expr::BinOp(k, l, r);
                    expr = Expr::BinOp(kind, Box::new(last), Box::new(expr));
                }
            } else {
                expr = Expr::BinOp(kind, Box::new(last), Box::new(right));
            }
            self.stack.push(expr);
        }
    }

    fn parse_bin_op(&mut self) -> Option<BinOpKind> {
        self.skip_whitespace();
        match self.lexer.peek().unwrap().kind {
            TokenKind::Eq => {
                self.lexer.next();
                Some(match self.lexer.peek().unwrap().kind {
                    TokenKind::Eq => {
                        self.lexer.next();
                        BinOpKind::Eq
                    }
                    _ => BinOpKind::Assign,
                })
            }
            TokenKind::Lt => {
                self.lexer.next();
                Some(match self.lexer.peek().unwrap().kind {
                    TokenKind::Eq => {
                        self.lexer.next();
                        BinOpKind::LtEq
                    }
                    _ => BinOpKind::Lt,
                })
            }
            TokenKind::Gt => {
                self.lexer.next();
                Some(match self.lexer.peek().unwrap().kind {
                    TokenKind::Eq => {
                        self.lexer.next();
                        BinOpKind::GtEq
                    }
                    _ => BinOpKind::Gt,
                })
            }
            TokenKind::Add => {
                self.lexer.next();
                Some(BinOpKind::Add)
            }
            TokenKind::Sub => {
                self.lexer.next();
                Some(BinOpKind::Sub)
            }
            TokenKind::Mul => {
                self.lexer.next();
                Some(match self.lexer.peek().unwrap().kind {
                    TokenKind::Mul => {
                        self.lexer.next();
                        BinOpKind::Pow
                    }
                    _ => BinOpKind::Mul,
                })
            }
            TokenKind::Div => Some(BinOpKind::Div),
            TokenKind::Percent => Some(BinOpKind::Mod),
            _ => None,
        }
    }

    fn print_error(&self, msg: &str, index: usize, len: usize) {
        eprintln!("{}:", msg);
        let (offset, line) = self.line_by_index(index);
        eprintln!("{}", line);
        eprintln!("{}{}", " ".repeat(offset), "^".repeat(len));
    }
    fn line_by_index(&self, index: usize) -> (usize, String) {
        let mut start = index;
        let mut end = index;
        while start > 0 {
            if let Some("\n") = self.code.get(start..=start) {
                break;
            }
            start -= 1;
        }
        while end < usize::MAX {
            if let Some("\n") | None = self.code.get(end..=end) {
                break;
            }
            end += 1;
        }
        (
            index - start,
            self.code.get(start..end).unwrap().to_string(),
        )
    }
    fn skip_whitespace(&mut self) {
        if let Some(Token {
            kind: TokenKind::Whitespace,
            ..
        }) = self.lexer.peek()
        {
            self.lexer.next();
        }
    }
}

fn escape_str(src: &str) -> String {
    let mut buf = String::new();
    iterate_str(src, |one, two| match one {
        Some('\\') => match two {
            Some('n') => buf.push('\n'),
            Some('r') => buf.push('\r'),
            Some('t') => buf.push('\t'),
            Some('\\') => buf.push('\\'),
            _ => (),
        },
        Some(ch) => buf.push(ch),
        None => (),
    });
    buf
}

fn iterate_str<CB>(s: &str, mut call_back: CB)
where
    CB: FnMut(Option<char>, Option<char>),
{
    let mut chars = s.chars();
    let mut one = chars.next();
    let mut two = chars.next();
    while one.is_some() {
        call_back(one, two);
        one = chars.next();
        two = chars.next();
    }
}
pub fn parse_based_f64(base: u32, num: &str) -> Option<f64> {
    let mut left: f64;
    let mut divider = 1f64;
    let mut right: f64;
    if num.contains('.') {
        let mut s = num.split('.');
        left = i32::from_str_radix(s.next()?, base).ok()? as f64;
        right = i32::from_str_radix(s.next()?, base).ok()? as f64;
        while divider < right {
            divider *= 10f64;
        }
        right /= divider;
        left += right;
    } else {
        left = i32::from_str_radix(num, base).ok()? as f64;
    }
    Some(left)
}

pub fn parse_based_f32(base: u32, num: &str) -> Option<f32> {
    let mut left: f32;
    let mut divider = 1f32;
    let mut right: f32;
    if num.contains('.') {
        let mut s = num.split('.');
        left = i32::from_str_radix(s.next()?, base).ok()? as f32;
        right = i32::from_str_radix(s.next()?, base).ok()? as f32;
        while divider < right {
            divider *= 10f32;
        }
        right /= divider;
        left += right;
    } else {
        left = i32::from_str_radix(num, base).ok()? as f32;
    }
    Some(left)
}
