use std::{
    fmt::{Debug, Display},
    io::{BufRead, BufReader, Read},
    str::Chars,
};

use self::LitKind::*;
use self::TokenKind::*;

#[derive(Debug, Clone)]
pub(crate) struct Token {
    pub(super) kind: TokenKind,
    pub(super) val: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TokenKind {
    LineComment,
    BlockComment,
    Ident,
    /// Literals kind:
    Lit {
        kind: LitKind,
    },
    /// "="
    Eq,
    /// "<"
    Lt,
    /// ">"
    Gt,
    /// "."
    Dot,
    /// "!"
    Not,
    /// "&"
    And,
    /// "|"
    Or,
    /// Delims like "{}","()","[]""
    OpenDelim {
        kind: DelimKind,
    },
    CloseDelim {
        kind: DelimKind,
    },
    /// "%"
    Percent,
    /// "$"
    Dollar,
    /// "#"
    Hash,
    /// "/"
    Div,
    /// "*"
    Mul,
    /// "+"
    Add,
    /// "-"
    Sub,
    /// ":"
    DoubleDot,
    /// "@"
    At,
    /// ";"
    Semi,
    /// "?"
    Question,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DelimKind {
    Bracket,
    Brace,
    Paren,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LitKind {
    Int,
    Float,
    Str,
    Bool,
}

impl Token {
    pub fn new(kind: TokenKind, val: String) -> Self {
        Self {
            kind,
            val: Some(val),
        }
    }
}
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{:#?}: {:#?}]", self.kind, self.val))
    }
}
pub struct Lexer {
    input: Cursor,
    cur_tok: Option<Token>,
}
impl Lexer {
    pub fn new(code: &'static str) -> Self {
        let l = Self {
            input: Cursor::new(code),
            cur_tok: None,
        };
        l.read_token();
        l
    }
    pub fn eof(&mut self) -> bool {
        self.input.eof()
    }
    pub fn peek(&mut self) -> Option<&Token> {
        if self.cur_tok.is_none() {
            self.read_token();
        }
        self.cur_tok.as_ref()
    }
    pub fn next(&self) -> Option<Token> {
        let tok = self.cur_tok;
        self.read_token();
        tok
    }
    pub fn read_token(&mut self) {
        if self.input.eof() {
            self.cur_tok = None;
            return;
        }
        let ch = self.input.next().unwrap();
        let tok_kind = match ch {
            '@' => At,
            '$' => Dollar,
            '&' => And,
            '|' => Or,
            ':' => DoubleDot,
            '.' => Dot,
            ';' => Semi,
            '+' => Add,
            '-' => Sub,
            '*' => Mul,
            '/' => self.read_div_or_comment(),
            '?' => Question,
            '!' => Not,
            '#' => Hash,
            '=' => Eq,
            '<' => Lt,
            '>' => Gt,
            '%' => Percent,
            '"' => self.read_double_quoted_string(),
            '\"' => self.read_single_quoted_string(),
            c @ '0'..='9' => self.read_number(c),
            c if is_id_start(c) => self.read_ident(c),
            _ => self.read_unkown(),
        };
    }

    fn eat_while<T>(&mut self, predicate: T)
    where
        T: FnMut([Option<char>; 2]) -> bool,
    {
        while predicate([self.input.peek(), self.input.preview()]) {
            self.input.next();
        }
    }
    fn read_number(&mut self, first: char) -> TokenKind {
        if first == '0' {
            if self.input.eof() {
                return TokenKind::Lit { kind: LitKind::Int };
            }
            match self.input.peek().unwrap() {
                'b' => self.eat_bin_number(),
                'o' => self.eat_oct_number(),
                'x' => self.eat_hex_number(),
                _ => self.eat_dec_number(),
            }
        } else {
            self.eat_dec_number()
        }
    }

    fn eat_dec_number(&mut self) -> TokenKind {
        self.eat_while(|[first, second]| match first {
            Some('0'..='9') => true,
            _ => false,
        });
        if let Some('.') = self.input.peek() {
            self.input.next();
            self.eat_while(|[first, second]| match first {
                Some('0'..='9') => true,
                _ => false,
            });
            let suff_off = self.input.get_len();
            self.eat_suffix();
            return Lit { kind: Float };
        }
        if let Some('u') | Some('i') = self.input.peek() {
            let suff_off = self.input.get_len();
            self.eat_suffix();
        }
        Lit { kind: Int }
    }
    fn eat_oct_number(&mut self) -> TokenKind {
        self.eat_while(|[first, second]| match first {
            Some('0'..='7') => true,
            _ => false,
        });
        if let Some('.') = self.input.peek() {
            self.input.next();
            self.eat_while(|[first, second]| match first {
                Some('0'..='7') => true,
                _ => false,
            });
            let suff_off = self.input.get_len();
            self.eat_suffix();
            return Lit { kind: Float };
        }
        if let Some('u') | Some('i') = self.input.peek() {
            let suff_off = self.input.get_len();
            self.eat_suffix();
        }
        Lit { kind: Int }
    }
    fn eat_bin_number(&mut self) -> TokenKind {
        self.eat_while(|[first, second]| match first {
            Some('0'..='1') => true,
            _ => false,
        });
        if let Some('.') = self.input.peek() {
            self.input.next();
            self.eat_while(|[first, second]| match first {
                Some('0'..='1') => true,
                _ => false,
            });
            let suff_off = self.input.get_len();
            self.eat_suffix();
            return Lit { kind: Float };
        }
        if let Some('u') | Some('i') = self.input.peek() {
            let suff_off = self.input.get_len();
            self.eat_suffix();
        }
        Lit { kind: Int }
    }
    fn eat_hex_number(&mut self) -> TokenKind {
        self.eat_while(|[first, second]| match first {
            Some('0'..='9' | 'a'..='f' | 'A'..='F') => true,
            _ => false,
        });
        if let Some('.') = self.input.peek() {
            self.input.next();
            self.eat_while(|[first, second]| {
                matches!(first, Some('0'..='1' | 'a'..='f' | 'A'..='F'))
            });
            let suff_off = self.input.get_len();
            self.eat_suffix();
            return Lit { kind: Float };
        }
        if let Some('u') | Some('i') = self.input.peek() {
            let suff_off = self.input.get_len();
            self.input.next();
            self.eat_suffix();
        }
        Lit { kind: Int }
    }
    fn eat_suffix(&mut self) {
        match self.input.peek() {
            Some('0'..='9') => {
                self.eat_while(|[ch, _]| matches!(ch, Some('0'..='9')));
            }
            Some('a'..='z') => self.eat_while(|[c, _]| matches!(c, Some('a'..='z'))),
        }
    }
    fn read_string(&mut self) {}
}

pub(crate) struct Cursor {
    len: usize,
    buf: Chars<'static>,
}

impl Cursor {
    pub fn new(buf: &'static str) -> Self {
        Self {
            len: 0,
            buf: buf.chars(),
        }
    }
    pub fn peek(&mut self) -> Option<char> {
        self.buf.clone().next()
    }
    pub fn next(&mut self) -> Option<char> {
        self.len += 1;
        self.buf.next()
    }
    pub fn preview(&mut self) -> Option<char> {
        let b = self.buf.clone();
        b.next();
        b.next()
    }
    pub fn get_len(&self) -> usize {
        self.len
    }
    pub fn reset_len(&mut self) {
        self.len = 0;
    }
    pub fn eof(&mut self) -> bool {
        self.buf.as_str().is_empty()
    }
}

pub struct Error {
    col: usize,
    line: usize,
    len: usize,
    line_str: String,
    msg: String,
}
impl Error {
    pub fn new(col: usize, line: usize, len: usize, line_str: String, msg: String) -> Self {
        Self {
            col,
            line,
            len,
            line_str,
            msg,
        }
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}({}:{})", self.msg, self.line, self.col))
    }
}
impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut marker = String::new();
        for _ in 0..self.col {
            marker.push(' ');
        }
        marker.push('^');
        f.write_fmt(format_args!(
            "{}({}:{})\n\n{}\n{}",
            self.msg,
            self.line + 1,
            self.col + 1,
            self.line_str,
            marker
        ))
    }
}
impl std::error::Error for Error {}
