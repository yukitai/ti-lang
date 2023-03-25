use std::{collections::HashMap, rc::Rc};

use crate::vm::types::object_type::TiObj;


#[derive(Debug, Clone)]
pub struct TiEnvironment {
  vars: HashMap<Rc<String>, TiObj>,
}

impl TiEnvironment {
  pub fn new() -> Self {
    Self { vars: HashMap::new() }
  }
  pub fn set(&mut self, name: Rc<String>, value: TiObj) {
    self.vars.insert(name, value);
  }
  pub fn move_out(&mut self, name: &Rc<String>) -> Option<TiObj> {
    self.vars.remove(name)
  }
  pub fn get(&mut self, name: &Rc<String>) -> Option<TiObj> {
    self.vars.get(name).and_then(|v| Some(v.clone()))
  }
}