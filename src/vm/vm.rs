use crate::backend::bytecode::{bytecode::TiByteCode};

use super::{types::{object_type::TiObj}, environment::environment::TiEnvironment};

pub struct TiVM<'a> {
  bytecode: TiByteCode<'a>,
  stack: Vec<TiObj>,
  environment: TiEnvironment,
}

impl<'a> TiVM<'a> {
  pub fn new(bytecode: TiByteCode<'a>) -> Self {
    Self { 
      bytecode, 
      stack: vec! [], 
      environment: TiEnvironment::new(),
    }
  }
  pub fn with_environment(bytecode: TiByteCode<'a>, environment: TiEnvironment) -> Self {
    Self { 
      bytecode, 
      stack: vec! [], 
      environment,
    }
  }
}

impl TiVM<'_> {
  pub fn push(&mut self, obj: TiObj) -> &Self {
    self.stack.push(obj);
    self
  }
  pub fn pop(&mut self) -> TiObj {
    self.stack.pop().unwrap()
  }

  pub fn execute(self) -> TiObj {
    let mut res = TiObj::List(Vec::new());
    while !self.bytecode.has() {}
    res
  }
}