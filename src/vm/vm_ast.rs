use std::rc::Rc;

use crate::{frontend::parser::ast::{AstNode, AstBlock, AstExpr}, build_ti_error};

use super::{types::{object_type::TiObj}, environment::environment::TiEnvironment};

pub struct TiVM {
  stack: Vec<TiObj>,
  environment: TiEnvironment,
}

impl TiVM {
  pub fn new() -> Self {
    Self { 
      stack: vec! [], 
      environment: TiEnvironment::new(),
    }
  }
  pub fn with_environment(environment: TiEnvironment) -> Self {
    Self { 
      stack: vec! [], 
      environment,
    }
  }
}

impl TiVM {
  pub fn push(&mut self, obj: TiObj) -> &Self {
    self.stack.push(obj);
    self
  }
  pub fn pop(&mut self) -> TiObj {
    self.stack.pop().unwrap()
  }

  fn execute_block(&mut self, ast_block: AstBlock) -> TiObj {
    let mut res = TiObj::BuildUnit();
    for stmt in ast_block.block {
      res = self.execute_ast(stmt);
    }
    res
  }
  
  fn execute_expr(&mut self, ast_expr: AstExpr) -> TiObj {
    match ast_expr {
        AstExpr::Add(x, y) => {
          let lhs = self.execute_expr(*x);
          let rhs = self.execute_expr(*y);
          lhs.add(&rhs)
        },
        AstExpr::Sub(x, y) => {
          let lhs = self.execute_expr(*x);
          let rhs = self.execute_expr(*y);
          lhs.sub(&rhs)
        },
        AstExpr::Mul(x, y) => {
          let lhs = self.execute_expr(*x);
          let rhs = self.execute_expr(*y);
          lhs.mul(&rhs)
        },
        AstExpr::Div(x, y) => {
          let lhs = self.execute_expr(*x);
          let rhs = self.execute_expr(*y);
          lhs.div(&rhs)
        },
        AstExpr::AddEq(x, y) => {
          todo!()
        },
        AstExpr::SubEq(x, y) => {
          todo!()
        },
        AstExpr::MulEq(x, y) => {
          todo!()
        },
        AstExpr::DivEq(x, y) => {
          todo!()
        },
        AstExpr::Eq(x, y) => {
          todo!()
        },
        AstExpr::Neq(x, y) => {
          todo!()
        },
        AstExpr::Grt(x, y) => {
          todo!()
        },
        AstExpr::Les(x, y) => {
          todo!()
        },
        AstExpr::Geq(x, y) => {
          todo!()
        },
        AstExpr::Leq(x, y) => {
          todo!()
        },
        AstExpr::Not(x, y) => {
          todo!()
        },
        AstExpr::Assign(x, y) => {
          todo!()
        },
        AstExpr::FnCall(mem, args) => {
          let caller = self.execute_expr(*mem);
          match caller {
            TiObj::Fn(fdef, fbody, environment) => {
              let mut vm = TiVM::with_environment(environment);
              let args: Vec<TiObj> = args.into_iter().map(|expr| self.execute_expr(expr)).collect();
              for idx in 0..args.len() {
                vm.environment.set(fdef.arguments[idx].name.clone(), args[idx].clone());
              }
              vm.execute_block(fbody)
            }
            _ => build_ti_error!(@err "type error: type `{:?}` is not callable.", caller.as_type()),
          }
        },
        AstExpr::Var(x) => {
          if let Some(res) = self.environment.get(&x) {
            res
          } else {
            build_ti_error!(@err "reference error: name `{}` is not declared or has already moved.", x)
          }
        },
        AstExpr::LiteralNum(x) => {
          TiObj::Num(x)
        },
        AstExpr::LiteralStr(x) => {
          TiObj::Str(x)
        },
        AstExpr::LiteralUnit(x) => {
          TiObj::List(x.into_iter().map(|expr| self.execute_expr(expr)).collect())
        },
        AstExpr::LiteralArray(x) => {
          TiObj::List(x.into_iter().map(|expr| self.execute_expr(expr)).collect())
        },
    }
  }

  fn execute_ast(&mut self, ast_node: AstNode) -> TiObj {
    match ast_node {
        AstNode::Program(program) => {
          for stmt in program.program {
            self.execute_ast(stmt);
          }
          TiObj::BuildUnit()
        },
        AstNode::Fn(fdef, fbody) => {
          let name = fdef.name.clone();
          let ti_fn = TiObj::Fn(fdef, fbody, self.environment.clone());
          self.environment.set(name, ti_fn.clone());
          ti_fn
        },
        AstNode::Impl(_, _) => unreachable!(),
        AstNode::Let(name, value) => {
          let res = self.execute_expr(value);
          self.environment.set(name, res);
          TiObj::BuildUnit()
        },
        AstNode::Expr(expr) => {
          self.execute_expr(expr)
        },
        AstNode::Empty => unreachable!(),
    }
  }

  pub fn execute(&mut self, ast_node: AstNode, entry: &str) -> TiObj {
    self.execute_ast(ast_node);

    if let Some(TiObj::Fn(_, entry_body, _)) = self.environment.move_out(&Rc::new(entry.to_string())) {
      self.execute_block(entry_body)
    } else {
      build_ti_error!(@err "runtime error: no entry.")
    }
  }
}