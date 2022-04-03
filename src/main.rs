mod error;
mod parser;

use parser::Parser;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut parser = Parser::new(
        r"
        let mut a = 0;
        0;
    ",
    );
    let expr = parser.parse_top();
    dbg!(expr);
    dbg!(parser.errors);
    Ok(())
}
