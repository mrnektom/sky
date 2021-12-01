extern crate regress;

use std::error::Error;
use regress::Regex;

pub struct Lexer {
  regex: Regex
}

impl Lexer {
  pub fn new() -> Result<Self, Box<dyn Error>> {
    let regs = vec![
      r#"(?<str>(?<q>["'`]).*\k<q>)"#,
      r#"(?<iden>\b[a-zA-Z_$#]{1,}[a-zA-Z0-9_$#]*\b)"#,
      r#"(?<nm>(0x?)?[0-9a-fA-F_])+"#,
      r#"(?<br>(\(|\)|\[|\]|\{|\}))"#,
      r#"(?<op>\*|\/|&&|\|\||>=|<=|<|>|=)"#,
      r#"(?<pnk>[,\.:])"#,
      r#"(?<nl>\r\n|\n\r|\n)"#,
      r#"(?<wh>\s+)"#,
      r#"(?<unk>.+?)"#
    ];
    let mut reg = String::from("(");
    reg.push_str(regs.join("|").as_str());
    reg.push(')');
    println!("{}",reg);
    let regex = Regex::new(reg.as_str())?;
  	Ok(Lexer { 
  	  regex
    })
  }
  pub fn lex(self,string: &str) -> Result<&mut Vec<(String, String)>, Box<dyn Error>> {
    let mut toks = Vec::new();
    for mat in self.regex.find_iter(string) {
      for (name, range) in mat.named_groups() {
        if name == "q" {
          continue;
        }
        match range {
          None => continue,
          _ => ()
        }
      	toks.push((name.to_string(),string[range.unwrap()].to_string()));
      }
    }
    Ok(&mut toks)
  }
}
