use crate::frontend::parser::parser::Parser;

pub struct Codegen {
  parser: Parser,
}

impl Codegen {
  pub fn new(parser: Parser) -> Self {
    Self { parser }
  }
}

impl Codegen {
  // this while be replaced with the struct `TiBytecode`
  pub fn bytecode(&self) -> String {
    format!("{:#?}\n{:#?}", self.parser.ast, self.parser.fn_def)
  }
}