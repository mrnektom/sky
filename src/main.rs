mod parser;

use parser::{lexer::Lexer, Parser};

use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut parser = Parser::new("10");
    parser.compute_expr();
    Ok(())
}
