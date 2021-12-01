mod ast;

use std::error::Error;
use std::io::{ Read };
use std::fs::File;
use crate::ast::lexer::Lexer;

fn main() -> Result<(), Box<dyn Error>> {
  let mut file = File::open("./script.sk")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  let lexer = Lexer::new()?;
  for (typel ,lexema) in lexer.lex(&contents)? {
    if lexema == "\n" {
      println!("{},'\\n'",typel);
      continue;
    }
  	println!("{}:'{}'",typel,lexema);
  }
  Ok(())
}
