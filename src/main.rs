mod parser;

use parser::{ast::Expr, Parser};

use std::{error::Error, mem::size_of_val};

fn main() -> Result<(), Box<dyn Error>> {
    let mut parser = Parser::new(r#"0b111111111"#);
    let expr = parser.parse_top();
    println!("{:#?}", expr);
    Ok(())
}
