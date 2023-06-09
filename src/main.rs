use compiler::gen;

use crate::parser::parse;

mod error;
mod parser;
mod analyzer;
mod compiler;

use std::io::prelude::*;
use std::{env::args, error::Error, fs::File};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = args().collect();
    let path: Option<String> = args.get(1).cloned();

    if let Some(p) = path {
        let mut file = File::open(p)?;
        let mut source = String::new();
        file.read_to_string(&mut source)?;
        let ast = parse(&source);
        match ast {
            Ok(ast) => {
                println!("{}", gen(ast));
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    } else {
        println!("No provided path/to/file.sk")
    }
    Ok(())
}
