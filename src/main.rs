mod parser;

use parser::lexer::Lexer;

use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("script.sky")?;
    let mut buf = String::new();
    BufReader::new(f).read_to_string(&mut buf)?;
    let mut lexer = Lexer::new(buf.as_str());
    while !lexer.eof() {
        let tok = lexer.next().unwrap();
        println!("{:?}", tok);
        println!("{}", buf[tok.index..tok.index + tok.size].to_string())
    }
    Ok(())
}
