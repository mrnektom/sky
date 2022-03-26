mod error;
mod parser;

use parser::Parser;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut parser = Parser::new(
        r"
    if 0 > 1 {
        4;
        5;
    } else 555
    ",
    );
    let expr = parser.parse_top();
    dbg!(expr);
    dbg!(parser.errors);
    Ok(())
}
