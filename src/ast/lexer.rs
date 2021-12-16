extern crate regress;

use std::{
    fmt::{Debug, Display},
    fs::File,
    io::{BufRead, BufReader, Read},
};

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
    pub fn from_code(code: &str) -> Self {
        Self {
            input: InputStream::from_string(code.to_string()),
            cur_tok: None,
            logger: Logger::new(LogLevel::Verbose),
        }
    }
    pub fn from_reader(reader: R) -> std::io::Result<Self> {
        Ok(Self {
            input: InputStream::from_reader(reader)?,
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

    pub fn read_next(&mut self) -> Result<&Token, crate::ast::lexer::Error> {
        println!("Reading next token...");
        self.read_while(Self::is_whitespace);
        if self.input.eof() {
            self.cur_tok = None;
        } else {
            let ch = self.input.peek();
            self.logger.verb(format!("Current char is \"{}\"", ch));

            if ch.is_whitespace() {
                self.logger.verb("Skiping whitespace");
                self.logger.verb("Skiping comment");
            }
            if ch == '/' && (self.input.preview() == '/' || self.input.preview() == '*') {
                self.skip_comment();
                self.read_next()?;
            } else if ch == '"' || ch == '\'' || ch == 'r' && self.input.preview() == '#' {
                println!("Reading string literal");
                self.read_string();
            } else if self.is_ident_start() {
                println!("Reading ident");
                self.read_ident();
            } else if ch.is_digit(10) {
                println!("Reading number");
                self.read_number();
            } else if ch.is_ascii_punctuation() && ch != '"' && ch != '\'' {
                println!("Reading punctuation symbol");
                self.read_punc();
            } else {
                println!("Current token is None");
                self.cur_tok = None;
            }
        }
        println!("Returning token");
        match self.cur_tok.as_ref() {
            None => Err(self.input.croak("Invalid token".to_string())),
            Some(tok) => Ok(tok),
        }
    }
    fn read_while<T>(&mut self, predicate: T) -> String
    where
        T: Fn(char) -> bool,
    {
        let mut s = String::new();
        while !self.input.eof() && predicate(self.input.peek()) {
            s.push(self.input.next());
        }
        s
    }
    fn is_whitespace(c: char) -> bool {
        c.is_whitespace()
    }
    fn read_number(&mut self) {
        let mut has_dot = false;
        self.cur_tok = Some(Token {
            kind: "NUMBER".to_string(),
            val: {
                let mut buf = String::new();
                while !self.input.eof() {
                    if self.input.peek().is_digit(10) {
                        buf.push(self.input.peek());
                    } else if self.input.peek() == '.' && !has_dot {
                        has_dot = true;
                    } else {
                        break;
                    }
                    self.input.next();
                }
                buf
            },
        });
    }
    fn read_string(&mut self) {
        let end = self.input.peek().to_string();
        self.cur_tok = Some(Token {
            kind: "STR_LIT".to_string(),
            val: self.read_escaped(end),
        });
    }
    fn read_escaped(&mut self, end: String) -> String {
        let mut escaped = false;
        let mut buf = String::new();
        let mut end_buf = String::new();
        buf.push(self.input.next());
        while !self.input.eof() {
            let ch = self.input.next();
            if escaped {
                buf.push(ch);
                escaped = false;
            } else if end_buf == end {
                break;
            } else if end.starts_with(ch) && end_buf.is_empty() {
                end_buf.push(ch);
                buf.push(ch)
            } else {
                buf.push(ch);
            }
        }
        buf
    }
    fn skip_comment(&self) {}
    fn is_ident_start(&mut self) -> bool {
        let ch = self.input.peek();
        ch.is_alphabetic() || ch == '_' || ch == '$'
    }
    fn read_ident(&mut self) {
        let mut buf = String::new();
        while !self.input.eof() {
            let ch = self.input.peek();
            if ch.is_alphabetic() || ch.is_digit(10) || ch == '_' {
                buf.push(ch)
            } else {
                break;
            }
            self.input.next();
        }
        self.cur_tok = Some(Token {
            kind: "IDENT".to_string(),
            val: buf,
        })
    }
    fn read_punc(&mut self) {
        let ch = self.input.peek();
        self.input.next();
        match ch {
            '-' | '+' | '/' | '*' | '=' => {
                let chp = self.input.peek();
                let mut buf = String::from(ch);
                if chp.is_ascii_punctuation() {
                    if ch == chp {
                        buf.push(ch);
                        self.input.next();
                    }
                }
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
                self.cur_tok = Some(Token {
                    kind: "PUNC".to_string(),
                    val: String::from(ch),
                })
            }
            _ => self.cur_tok = None,
        }
    }
}

struct InputStream<R: Read> {
    index: usize,
    line: usize,
    buf: String,
    reader: Option<BufReader<R>>,
}

impl<R: Read> InputStream<R> {
    pub fn from_string(buf: String) -> Self {
        Self {
            index: 0,
            line: 0,
            buf,
            reader: None,
        }
    }
    pub fn from_reader(reader: R) -> std::io::Result<Self> {
        let mut buf = String::new();
        let mut reader = BufReader::new(reader);
        reader.read_line(&mut buf)?;
        Ok(InputStream {
            index: 0,
            line: 0,
            buf,
            reader: Some(reader),
        })
    }
    pub fn peek(&mut self) -> char {
        self.update_buf();
        match self.buf.chars().nth(self.index) {
            None => '\u{00}',
            Some(c) => c,
        }
    }
    pub fn next(&mut self) -> char {
        let c = self.peek();
        self.index += 1;
        if c == '\n' {
            self.line += 1;
        }
        c
    }
    pub fn preview(&mut self) -> char {
        self.update_buf();
        match self.buf.chars().nth(self.index + 1) {
            None => '\u{00}',
            Some(c) => c,
        }
    }
    pub fn prewiew_count(&mut self, n: usize) -> String {
        self.update_buf();
        let mut buf = String::new();
        let mut i = self.index.clone();
        let chs = self.buf.clone();
        let mut chs = chs.chars();
        while !Self::eof(self) {
            match chs.nth(i) {
                None => {
                    if !self.update_buf() {
                        break;
                    }
                }
                Some(ch) => buf.push(ch),
            }
            i += 1;
        }
        buf
    }
    pub fn eof(&mut self) -> bool {
        match self.reader {
            None => self.index >= self.buf.len(),
            Some(_) => !self.update_buf(),
        }
    }
    pub fn croak(&self, msg: String) -> Error {
        println!("Formatting error message");
        let mut line = self.buf.clone();
        let mut end = 0;
        let mut i = self.index.clone();
        'lp: loop {
            match line.find('\n') {
                None => break 'lp,
                Some(index) => {
                    if index < i {
                        line = line[index + 1..].to_string();
                        i -= index + 1;
                    } else {
                        end = index;
                        break 'lp;
                    }
                }
            }
        }
        line = line[..end].to_string();
        println!("Returning error");
        Error {
            col: i,
            line: self.line,
            len: 1,
            line_str: line,
            msg,
        }
    }
    fn update_buf(&mut self) -> bool {
        if self.index >= self.buf.len() {
            self.index = 0;
            match self.reader.as_mut() {
                Some(reader) => match reader.read_line(&mut self.buf) {
                    Ok(_) => {
                        self.line += 1;
                        true
                    }
                    Err(_) => false,
                },
                None => false,
            }
        } else {
            true
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
