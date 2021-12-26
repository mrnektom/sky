mod ast;
mod logger;

use ast::lexer::{self, InputStream};

use crate::{ast::lexer::Lexer, logger::LogLevel};
use std::{error::Error, fs::File, io::{Write, stdout}};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("script.sky")?;
    let mut lexer = Lexer::from_reader(f)?;
    while !lexer.eof() {
    println!("{}",lexer.next().unwrap());
    }
    Ok(())
}
