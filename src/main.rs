mod parser;

use parser::Parser;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut parser = Parser::new(r"6(1,0f64,1,2)");
    let expr = parser.parse_top();
    dbg!(expr);
    dbg!(parser.errors);
    Ok(())
}
