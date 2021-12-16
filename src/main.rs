mod ast;
mod logger;

use crate::{ast::lexer::Lexer, logger::LogLevel};
use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("script.sk")?;
    let mut buf = String::new();
    let mut reader = BufReader::new(f);
    reader.read_to_string(&mut buf)?;
    let mut lexer: Lexer<File> = Lexer::from_code(buf.as_str());
    while !lexer.eof() {
        let token = lexer.read_next();
        println!("Reciving token");
        println!("{:?}", token);
    }
    println!("{}", LogLevel::Verbose >= LogLevel::Error);
    Ok(())
}
