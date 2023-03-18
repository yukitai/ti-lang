use crate::frontend::parser::parser::Parser;

pub struct Codegen<'a> {
  parser: Parser<'a>,
}

impl<'a> Codegen<'a> {
  pub fn new(parser: Parser<'a>) -> Self {
    Self { parser }
  }
}

impl<'a> Codegen<'a> {
  // this while be replaced with the struct `TiBytecode`
  pub fn bytecode(&self) -> String {
    format!("{:#?}\n{:#?}", self.parser.ast, self.parser.fn_def)
  }
}