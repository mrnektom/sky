use std::{
    fmt::{Debug, Display},
    str::Chars,
};

use self::LitKind::*;
use self::TokenKind::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub size: usize,
    pub index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
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
    /// ","
    Comma,
    /// "!"
    Not,
    /// "&"
    And,
    /// "|"
    Or,
    /// "^"
    Caret,
    /// s like "{}","()","[]""
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
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
    Colon,
    /// "@"
    At,
    /// ";"
    Semi,
    /// "?"
    Question,

    Whitespace,

    Unkown,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LitKind {
    Num {
        base: Option<NumBase>,
        suff_off: Option<usize>,
    },
    Str,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NumBase {
    Bin,
    Oct,
    Hex,
}
impl Into<u32> for NumBase {
    fn into(self) -> u32 {
        match self {
            Self::Bin => 2,
            Self::Oct => 8,
            Self::Hex => 16,
        }
    }
}
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{:#?}: {:#?}]", self.kind, self.size))
    }
}

fn is_id_start(ch: char) -> bool {
    matches!(ch,'a'..='z'|'A'..='Z'|'_')
}

fn is_id_continue(ch: char) -> bool {
    matches!(ch, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '#' | '$' | '@')
}
#[derive(Debug)]
pub struct Lexer<'a> {
    input: Cursor<'a>,
    cur_tok: Option<Token>,
    code: &'a str,
}
impl<'a> Lexer<'a> {
    pub fn new(code: &'a str) -> Self {
        let mut l = Self {
            input: Cursor::new(code),
            cur_tok: None,
            code,
        };
        l.cur_tok = l.read_token();
        l
    }
    pub fn get_str(&self, len: usize) -> Option<&str> {
        let index = self.cur_tok?.index;
        self.code.get(index..index + len)
    }
    pub fn get_tok(&self) -> Option<&str> {
        let tok = self.peek()?;
        self.code.get(tok.index..tok.index + tok.size)
    }
    pub fn eof(&mut self) -> bool {
        self.input.eof() && self.cur_tok.is_none()
    }
    pub fn peek(&self) -> Option<Token> {
        self.cur_tok.clone()
    }
    pub fn next(&mut self) -> Option<Token> {
        let tok = self.cur_tok.clone();
        self.cur_tok = self.read_token();
        tok
    }
    fn read_token(&mut self) -> Option<Token> {
        if self.input.eof() {
            self.cur_tok = None;
            return None;
        }
        let ch = self.input.next().unwrap();
        let tok_kind = match ch {
            '@' => At,
            '$' => Dollar,
            '&' => And,
            '|' => Or,
            '^' => Caret,
            ':' => Colon,
            '.' => Dot,
            ',' => Comma,
            '(' => OpenParen,
            ')' => CloseParen,
            '[' => OpenBracket,
            ']' => CloseBracket,
            '{' => OpenBrace,
            '}' => CloseBrace,
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
            '\'' => self.read_single_quoted_string(),
            c @ '0'..='9' => self.read_number(c),
            c if is_id_start(c) => self.read_ident(),
            c if c.is_whitespace() => self.eat_whitespace(),
            _ => Unkown,
        };
        let token = Token {
            kind: tok_kind,
            index: self.input.get_index() - self.input.get_len(),
            size: self.input.get_len(),
        };
        self.input.reset_len();
        Some(token)
    }

    fn eat_while<T>(&mut self, mut predicate: T, skip: u32)
    where
        T: FnMut(&mut Self, Option<char>, Option<char>) -> bool,
    {
        let mut first = self.input.peek();
        let mut second = self.input.preview();
        while predicate(self, first, second) && !self.eof() {
            self.input.next();
            first = self.input.peek();
            second = self.input.preview();
        }
        if skip > 0 {
            for _ in 0..skip {
                self.input.next();
            }
        }
    }
    fn read_number(&mut self, first: char) -> TokenKind {
        if first == '0' {
            if self.input.eof() {
                return TokenKind::Lit {
                    kind: LitKind::Num {
                        base: None,
                        suff_off: None,
                    },
                };
            }
            match self.input.peek().unwrap() {
                'b' => self.eat_bin_number(),
                'o' => self.eat_oct_number(),
                'x' => self.eat_hex_number(),
                _ => self.eat_number(),
            }
        } else {
            self.eat_number()
        }
    }

    fn eat_number(&mut self) -> TokenKind {
        self.eat_while(
            |_, first, _| match first {
                Some('0'..='9') => true,
                _ => false,
            },
            0,
        );
        if let (Some('.'), Some('0'..='9')) = (self.input.peek(), self.input.preview()) {
            self.input.next();
            self.eat_while(
                |_, first, _| match first {
                    Some('0'..='9') => true,
                    _ => false,
                },
                0,
            );
            let suff_off = self.input.get_len();
            self.eat_num_suffix();
            return Lit {
                kind: Num {
                    base: None,
                    suff_off: Some(suff_off),
                },
            };
        }
        if let Some('u') | Some('i') | Some('f') = self.input.peek() {
            let suff_off = self.input.get_len();
            self.eat_num_suffix();
            return Lit {
                kind: Num {
                    base: None,
                    suff_off: Some(suff_off),
                },
            };
        }
        Lit {
            kind: Num {
                base: None,
                suff_off: None,
            },
        }
    }
    fn eat_oct_number(&mut self) -> TokenKind {
        self.input.next();
        self.eat_while(
            |_, first, _| match first {
                Some('0'..='7') => true,
                _ => false,
            },
            0,
        );
        if let (Some('.'), Some('0'..='7')) = (self.input.peek(), self.input.preview()) {
            self.input.next();
            self.eat_while(
                |_, first, _| match first {
                    Some('0'..='7') => true,
                    _ => false,
                },
                0,
            );
            let suff_off = self.input.get_len();
            self.eat_num_suffix();
            return Lit {
                kind: Num {
                    base: Some(NumBase::Oct),
                    suff_off: Some(suff_off),
                },
            };
        }
        if let Some('u') | Some('i') | Some('f') = self.input.peek() {
            let suff_off = self.input.get_len();
            self.eat_num_suffix();
            return Lit {
                kind: Num {
                    base: Some(NumBase::Oct),
                    suff_off: Some(suff_off),
                },
            };
        }
        Lit {
            kind: Num {
                base: Some(NumBase::Oct),
                suff_off: None,
            },
        }
    }
    fn eat_bin_number(&mut self) -> TokenKind {
        self.input.next();
        self.eat_while(
            |_, first, _| match first {
                Some('0'..='1') => true,
                _ => false,
            },
            0,
        );
        if let (Some('.'), Some('0'..='1')) = (self.input.peek(), self.input.preview()) {
            self.input.next();
            self.eat_while(
                |_, first, _| match first {
                    Some('0'..='1') => true,
                    _ => false,
                },
                0,
            );
            let suff_off = self.input.get_len();
            self.eat_num_suffix();
            return Lit {
                kind: Num {
                    base: Some(NumBase::Bin),
                    suff_off: Some(suff_off),
                },
            };
        }
        if let Some('u') | Some('i') | Some('f') = self.input.peek() {
            let suff_off = self.input.get_len();
            self.eat_num_suffix();
            return Lit {
                kind: Num {
                    base: Some(NumBase::Bin),
                    suff_off: Some(suff_off),
                },
            };
        }
        Lit {
            kind: Num {
                base: Some(NumBase::Bin),
                suff_off: None,
            },
        }
    }
    fn eat_hex_number(&mut self) -> TokenKind {
        self.input.next();
        self.eat_while(
            |_, first, _| match first {
                Some('0'..='9' | 'a'..='f' | 'A'..='F') => true,
                _ => false,
            },
            0,
        );
        if let (Some('.'), Some('0'..='9' | 'a'..='f' | 'A'..='F')) =
            (self.input.peek(), self.input.preview())
        {
            self.input.next();
            self.eat_while(
                |_, first, _| matches!(first, Some('0'..='9' | 'a'..='f' | 'A'..='F')),
                0,
            );
            let val = self
                .code
                .get(self.input.get_index() - self.input.get_len()..self.input.get_index())
                .unwrap();
            if val.ends_with("f64") || val.ends_with("f32") {
                return Lit {
                    kind: Num {
                        base: Some(NumBase::Hex),
                        suff_off: Some(self.input.get_len() - 3),
                    },
                };
            }
            if let Some('u') | Some('i') | Some('f') = self.input.peek() {
                let suff_off = self.input.get_len();
                self.eat_num_suffix();
                return Lit {
                    kind: Num {
                        base: Some(NumBase::Hex),
                        suff_off: Some(suff_off),
                    },
                };
            }
            self.eat_num_suffix();
            return Lit {
                kind: Num {
                    base: Some(NumBase::Hex),
                    suff_off: None,
                },
            };
        }
        if let Some('u') | Some('i') | Some('f') = self.input.peek() {
            let suff_off = self.input.get_len();
            self.eat_num_suffix();
            return Lit {
                kind: Num {
                    base: Some(NumBase::Hex),
                    suff_off: Some(suff_off),
                },
            };
        }
        let val = self
            .code
            .get(self.input.get_index()..self.input.get_index() + self.input.get_len())
            .unwrap();
        println!("{}", val.len());
        if val.ends_with("f64") || val.ends_with("f32") {
            let suff_off = self.input.get_index() - 3;
            return Lit {
                kind: Num {
                    base: Some(NumBase::Hex),
                    suff_off: Some(suff_off),
                },
            };
        }

        Lit {
            kind: Num {
                base: Some(NumBase::Hex),
                suff_off: None,
            },
        }
    }
    fn eat_num_suffix(&mut self) {
        if let Some('u' | 'i' | 'f') = self.input.peek() {
            self.input.next();
            match self.input.peek() {
                Some('0'..='9') => {
                    self.eat_while(|_, ch, _| matches!(ch, Some('0'..='9')), 0);
                }
                Some('a'..='z') => self.eat_while(|_, c, _| matches!(c, Some('a'..='z')), 0),
                _ => (),
            }
        }
    }
    fn read_double_quoted_string(&mut self) -> TokenKind {
        self.eat_while(
            |_, first, second| match second {
                Some('"') => matches!(first, Some('\\')),
                _ => true,
            },
            2,
        );
        Lit { kind: Str }
    }
    fn read_single_quoted_string(&mut self) -> TokenKind {
        self.eat_while(
            |_, first, second| match second {
                Some('\'') => matches!(first, Some('\\')),
                _ => true,
            },
            0,
        );
        Lit { kind: Str }
    }
    fn read_div_or_comment(&mut self) -> TokenKind {
        match self.input.peek() {
            Some('*') => self.eat_block_comment(),
            Some('/') => self.eat_line_comment(),
            _ => Div,
        }
    }
    fn eat_line_comment(&mut self) -> TokenKind {
        self.eat_while(
            |s, first, second| match first {
                Some('\n') => {
                    if let Some('\r') = second {
                        s.input.next();
                    }
                    false
                }
                _ => true,
            },
            1,
        );
        LineComment
    }
    fn eat_block_comment(&mut self) -> TokenKind {
        self.eat_while(
            |_, first, second| match second {
                Some('/') => !matches!(first, Some('*')),
                _ => true,
            },
            2,
        );
        BlockComment
    }
    fn read_ident(&mut self) -> TokenKind {
        self.eat_while(
            |_, first, _| match first {
                Some(ch) => is_id_continue(ch),
                None => false,
            },
            0,
        );
        Ident
    }
    pub fn eat_whitespace(&mut self) -> TokenKind {
        self.eat_while(
            |_, ch, _| {
                if let Some(ch) = ch {
                    ch.is_whitespace()
                } else {
                    false
                }
            },
            0,
        );
        Whitespace
    }
}
#[derive(Debug)]
pub(crate) struct Cursor<'a> {
    len: usize,
    index: usize,
    buf: Chars<'a>,
}

impl<'a> Cursor<'a> {
    pub fn new(buf: &'a str) -> Self {
        Self {
            len: 0,
            index: 0,
            buf: buf.chars(),
        }
    }
    pub fn peek(&self) -> Option<char> {
        self.buf.clone().next()
    }
    pub fn next(&mut self) -> Option<char> {
        let c = self.buf.next();
        self.len += c.unwrap().to_string().len();
        self.index += c.unwrap().to_string().len();
        c
    }
    pub fn preview(&self) -> Option<char> {
        let mut b = self.buf.clone();
        b.next();
        b.next()
    }
    pub fn get_len(&self) -> usize {
        self.len
    }
    pub fn reset_len(&mut self) {
        self.len = 0;
    }
    pub fn get_index(&self) -> usize {
        self.index
    }
    pub fn eof(&mut self) -> bool {
        self.buf.as_str().is_empty()
    }
}
#[cfg(test)]
mod tests {

    use crate::parser::lexer::{is_id_continue, Token, TokenKind};

    use super::{Cursor, Lexer};
    #[test]
    fn is_id_continue_test() {
        assert!(!is_id_continue(' '));
        assert!(is_id_continue('_'));
    }

    #[test]
    fn lexer_test() {
        let mut lexer = Lexer::new("let s = 'str'");
        let token = lexer.next();
        assert_eq!(
            token,
            Some(Token {
                kind: TokenKind::Ident,
                index: 0,
                size: 3
            })
        );
    }
    #[test]
    fn cursor_test() {
        let mut cursor = Cursor::new("abc");
        assert_eq!(Some('a'), cursor.peek());
        assert_eq!(Some('b'), cursor.preview());
        assert_eq!(Some('a'), cursor.next());
        assert_eq!(Some('b'), cursor.peek());
    }
}
