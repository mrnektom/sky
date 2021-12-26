use std::{fmt::Display, fs::File, io::{BufReader, Read}};

use super::lexer::Lexer;

pub struct Parser<R: Read> {
    lexer: Lexer<R>,
}

impl<R: Read> Parser<R> {
    pub fn from_path(path: &str) -> Result<Parser<File>, Box<dyn std::error::Error>> {
        let reader = File::open(path)?;
        let lexer = Lexer::from_reader(reader)?;
        Ok(Parser::<File> {
            lexer
        })
    }
    fn is_punc(&mut self, punc: &str) -> bool {
        if self.lexer.peek().is_none() {
            return false;
        }
        let tok = self.lexer.peek().unwrap();
        if tok.kind().eq("PUNC") && tok.val().eq(punc) {
            return true;
        }
        false
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
