use std::fmt;

pub struct Sky<'a> {
  globals: &'a Globals
}

impl Sky<'static> {
  pub fn new() -> Sky<'static> {
  	Sky{
  	  globals: &Globals{}
  	}
  }
}

pub struct Globals {
  
}

impl fmt::Display for Sky<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
  	write!(f, "Sky")
  }
}
