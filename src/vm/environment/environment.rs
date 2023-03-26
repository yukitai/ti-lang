use std::{collections::HashMap, rc::Rc};

use crate::vm::types::object_type::TiObj;

#[derive(Debug, Clone)]
pub struct TiEnvironment {
    vars: Vec<HashMap<Rc<String>, TiObj>>,
}

impl TiEnvironment {
    pub fn new() -> Self {
        Self {
            vars: vec![HashMap::new()],
        }
    }
    pub fn create_frame(&mut self) {
        self.vars.push(HashMap::new());
    }
    pub fn with(&mut self, mut environment: TiEnvironment) -> &Self {
        self.vars.append(&mut environment.vars);
        self
    }
    pub fn remove_frame(&mut self) {
        self.vars.pop();
    }
    pub fn set(&mut self, name: Rc<String>, value: TiObj) {
        self.vars.last_mut().unwrap().insert(name, value);
    }
    pub fn set_v(&mut self, name: Rc<String>, value: TiObj) {
        let mut i = self.vars.len();
        while i > 0 {
            i -= 1;
            let v = self.vars.get_mut(i).unwrap();
            if let Some(_) = v.get(&name) {
                v.insert(name, value);
                return
            }
        }
    }
    pub fn move_out(&mut self, name: &Rc<String>) -> Option<TiObj> {
        let mut i = self.vars.len();
        while i > 0 {
            i -= 1;
            if let Some(v) = self.vars.get_mut(i).unwrap().remove(name) {
                return Some(v);
            }
        }
        None
    }
    pub fn get(&self, name: &Rc<String>) -> Option<TiObj> {
        // self.vars.get(name).and_then(|v| Some(v.clone()))
        let mut i = self.vars.len();
        while i > 0 {
            i -= 1;
            if let Some(v) = self.vars.get(i).unwrap().get(name) {
                return Some(v.clone());
            }
        }
        None
    }
}
