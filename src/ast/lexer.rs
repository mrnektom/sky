use std::{fmt::{Debug, Display}, io::{BufRead, BufReader, Read}};

use crate::logger::{LogLevel, Logger};

#[derive(Debug, Clone)]
pub struct Token {
    kind: String,
    val: String,
}
impl Token {
    pub fn new(kind: String, val: String) -> Self {
        Self { kind, val }
    }
    pub fn kind(&self) -> String {
        self.kind.clone()
    }
    pub fn val(&self) -> String {
        self.val.clone()
    }
}
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{}: {}]", self.kind, self.val))
    }
}
pub struct Lexer<R: Read> {
    input: InputStream<R>,
    cur_tok: Option<Token>,
    logger: Logger,
}
impl<R: Read> Lexer<R> {
    pub fn from_code(code: &'static str) -> Self {
        Self {
            input: InputStream::from_string(code),
            cur_tok: None,
            logger: Logger::new(LogLevel::Verbose),
        }
    }
    pub fn from_reader(reader: R) -> std::io::Result<Lexer<R>>{
        Ok(Lexer::<R> {
            input: InputStream::<R>::from_reader(reader)?,
            cur_tok: None,
            logger: Logger::new(LogLevel::Log),
        })
    }
    pub fn eof(&mut self) -> bool {
        self.input.eof()
    }
    pub fn peek(&mut self) -> Option<&Token> {
        match &self.cur_tok {
            None => {
                self.read_next();
                self.cur_tok.as_ref()
            }
            Some(_) => self.cur_tok.as_ref(),
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        let mut tok = self.cur_tok.clone();
        if tok.is_none() {
            self.read_next();
            tok = self.cur_tok.clone();
        }
        self.read_next();
        tok
    }

    pub fn read_next(&mut self) {
        self.read_while(Self::is_whitespace);
        if self.input.eof() {
            self.cur_tok = None;
        } else {
            let ch = match self.input.peek() {
                Some(ch) => ch.clone(),
                None => {
                    self.cur_tok = None;
                    return;
                }
            };
            self.logger.verb(format!("Current char is \"{}\"", ch));

            if ch.is_whitespace() {
                self.read_while(char::is_whitespace);
            }
            if ch == '/' && (self.input.preview() == Some(&'/') || self.input.preview() == Some(&'*')) {
                self.skip_comment();
                self.read_next();
            } else if ch == '"' || ch == '\'' || ch == 'r' && self.input.preview() == Some(&'#') {
                self.read_string();
            } else if self.is_ident_start() {
                self.read_ident();
            } else if ch.is_digit(10) {
                self.read_number();
            } else if ch.is_ascii_punctuation() && ch != '"' && ch != '\'' {
                self.read_punc();
            } else {
                self.cur_tok = None;
            }
        }
    }
    fn read_while<T>(&mut self, predicate: T) -> String
    where
        T: Fn(char) -> bool,
    {
        let mut s = String::new();
        'lp: while !self.input.eof()
            && predicate(match self.input.peek() {
                Some(ch) => ch.clone(),
                None => {
                    break 'lp;
                }
            })
        {
            let ch = match self.input.next() {
                Some(ch) => ch,
                None => {
                    break;
                }
            };
            s.push(ch.clone());
        }
        s
    }
    fn is_whitespace(c: char) -> bool {
        c.is_whitespace()
    }
    fn read_number(&mut self) {
        let mut has_dot = false;
        let mut buf = String::new();
        while !self.input.eof() {
            let ch = self.input.peek().unwrap().clone();
            if ch.is_ascii_digit() {
                buf.push(ch);
            } else if ch == '.' && !has_dot {
                has_dot = true;
                buf.push(ch);
            } else {
                break;
            }
            self.input.next();
        }
        self.cur_tok = Some(Token {
            kind: "NUMBER".to_string(),
            val: buf
        });
    }
    fn read_string(&mut self) {
        let end = match self.input.peek() {
            Some(ch) => ch.to_string(),
            None => {
                self.cur_tok = None;
                return;
            }
        };
        self.cur_tok = Some(Token {
            kind: "STR_LIT".to_string(),
            val: self.read_escaped(end),
        });
    }
    fn read_escaped(&mut self, end: String) -> String {
        let mut escaped = false;
        let mut buf = String::new();
        let mut end_buf = String::new();
        buf.push(match self.input.next() {
            Some(ch) => ch.clone(),
            None => {
                return buf;
            }
        });
        while !self.input.eof() {
            let ch = match self.input.next() {
                Some(ch) => ch,
                None => {
                    break;
                }
            };
            if escaped {
                buf.push(ch.clone());
                escaped = false;
            } else if end_buf == end {
                break;
            } else if end.starts_with(ch.clone()) && end_buf.is_empty() {
                end_buf.push(ch.clone());
                buf.push(ch.clone())
            } else {
                buf.push(ch.clone());
            }
        }
        buf
    }
    fn skip_comment(&self) {}
    fn is_ident_start(&mut self) -> bool {
        let ch = match self.input.peek() {
            Some(ch) => ch.clone(),
            None => {
                return false;
            }
        };
        ch.is_alphabetic() || ch == '_' || ch == '$'
    }
    fn read_ident(&mut self) {
        let mut buf = String::new();
        while !self.input.eof() {
            let ch = self.input.peek().unwrap();
            if ch.is_alphabetic() || ch.is_ascii_digit() {
                buf.push(ch.clone());
            } else {
                break;
            }
            self.input.next();
        }
        self.cur_tok = Some(Token {
            kind: "IDENT".to_string(),
            val: buf
        })
    }
    fn read_punc(&mut self) {
        let ch = self.input.peek();
        if ch.is_none() {
            return;
        }
        let ch = ch.unwrap().clone();
        match ch {
            '-' | '+' | '/' | '*' | '=' => {
                self.input.next();
                let chp = self.input.peek();
                let mut buf = String::from(ch);
                match chp {
                    Some(chp) => {
                        if chp.is_ascii_punctuation() && chp == &ch {
                            buf.push(chp.clone());
                        }
                    }
                    None => (),
                }
                self.input.next();
                self.cur_tok = Some(Token {
                    kind: "OP".to_string(),
                    val: buf,
                });
            }
            '<' | '>' => {
                self.cur_tok = Some(Token {
                    kind: "OP".to_string(),
                    val: ch.to_string(),
                });
            }
            '{' | '}' | '(' | ')' | '[' | ']' | ',' | '.' | ';' | ':' => {
                self.input.next();
                self.cur_tok = Some(Token {
                    kind: "PUNC".to_string(),
                    val: String::from(ch),
                })
            }
            _ => self.cur_tok = None,
        }
    }
}

pub(crate) struct InputStream<R: Read> {
    index: usize,
    line: usize,
    buf: Vec<char>,
    reader: Option<BufReader<R>>,
}

impl<R: Read> InputStream<R> {
    pub fn from_string(buf: &'static str) -> Self {
        Self {
            index: 0,
            line:0,
            buf: buf.chars().collect(),
            reader: None,
        }
    }
    pub fn from_reader(reader: R) -> std::io::Result<InputStream<R>>{
        let mut buf = String::new();
        let mut reader: BufReader<R> = BufReader::new(reader);
        reader.read_line(&mut buf)?;
        let r: InputStream<R> = InputStream {
            index: 0,
            line: 0,
            buf: buf.chars().collect(),
            reader: Some(reader),
        };
        Ok(r)
    }
    pub fn peek(&mut self) -> Option<&char> {
        if self.buf.get(self.index).is_none() {
            self.update_buf();
        }
        self.buf.get(self.index)
    }
    pub fn next(&mut self) -> Option<&char> {
        if self.buf.get(self.index).is_none() {
            self.update_buf();
            let r = self.buf.get(self.index);
            self.index+=1;
            return r;
        }
        let r = self.buf.get(self.index);
        self.index+=1;
        r
    }
    pub fn preview(&mut self) -> Option<&char> {
        if self.buf.get(self.index+1).is_none() {
            self.update_buf();
        }
        self.buf.get(self.index+1)
    }
    pub fn eof(&mut self) -> bool {
        if self.reader.is_none() {
            self.index >= self.buf.len()
        } else if self.index < self.buf.len() {
            false
        } else {
            let _ = !self.update_buf();
            self.peek().is_none()
        }
    }
    pub fn croak(&self, msg: &str) -> Error {
        let iter = self.buf.iter();
        let line = String::from_iter(iter);
        let mut line = line.as_str();
        let mut end = 0;
        let mut i = self.index.clone();
        'lp: loop {
            match line.find('\n') {
                None => break 'lp,
                Some(index) => {
                    if index < i {
                        line = &line[index + 1..];
                        i -= index + 1;
                    } else {
                        end = index;
                        break 'lp;
                    }
                }
            }
        }
        line = &line[..end];
        Error {
            col: i,
            line: self.line,
            len: 1,
            line_str: line.to_string(),
            msg: msg.to_string(),
        }
    }
    fn update_buf(&mut self) -> bool {
        if self.index < self.buf.len() {
            true
        } else if self.reader.is_none() {
            false
        } else {
            let mut buf = String::new();
            let reader = self.reader.as_mut().unwrap();
            let success = reader.read_line(&mut buf).is_ok();
            self.index = 0;
            self.buf = buf.chars().collect();
            success
        }
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
