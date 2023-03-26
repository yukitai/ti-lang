use std::rc::Rc;

use crate::{
    build_ti_error,
    frontend::parser::ast::{AstBlock, AstExpr, AstNode},
};

use super::{environment::environment::TiEnvironment, types::object_type::TiObj};

pub struct TiVM {
    pub environment: TiEnvironment,
}

impl TiVM {
    pub fn new() -> Self {
        Self {
            environment: TiEnvironment::new(),
        }
    }
    pub fn with_environment(environment: TiEnvironment) -> Self {
        Self { environment }
    }
}

impl TiVM {
    fn execute_block(&mut self, ast_block: AstBlock) -> TiObj {
        self.environment.create_frame();
        let mut res = TiObj::build_unit();
        for stmt in ast_block.block {
            res = self.execute_ast(stmt);
        }
        self.environment.remove_frame();
        res
    }

    fn execute_expr(&mut self, ast_expr: AstExpr) -> TiObj {
        match ast_expr {
            AstExpr::Add(x, y) => {
                let lhs = self.execute_expr(*x);
                let rhs = self.execute_expr(*y);
                lhs.add(&rhs)
            }
            AstExpr::Sub(x, y) => {
                let lhs = self.execute_expr(*x);
                let rhs = self.execute_expr(*y);
                lhs.sub(&rhs)
            }
            AstExpr::Mul(x, y) => {
                let lhs = self.execute_expr(*x);
                let rhs = self.execute_expr(*y);
                lhs.mul(&rhs)
            }
            AstExpr::Div(x, y) => {
                let lhs = self.execute_expr(*x);
                let rhs = self.execute_expr(*y);
                lhs.div(&rhs)
            }
            AstExpr::AddEq(x, y) => {
                todo!()
            }
            AstExpr::SubEq(x, y) => {
                todo!()
            }
            AstExpr::MulEq(x, y) => {
                todo!()
            }
            AstExpr::DivEq(x, y) => {
                todo!()
            }
            AstExpr::Eq(x, y) => {
                let lhs = self.execute_expr(*x);
                let rhs = self.execute_expr(*y);
                lhs.eq(&rhs)
            }
            AstExpr::Neq(x, y) => {
                let lhs = self.execute_expr(*x);
                let rhs = self.execute_expr(*y);
                lhs.neq(&rhs)
            }
            AstExpr::Grt(x, y) => {
                let lhs = self.execute_expr(*x);
                let rhs = self.execute_expr(*y);
                lhs.grt(&rhs)
            }
            AstExpr::Les(x, y) => {
                let lhs = self.execute_expr(*x);
                let rhs = self.execute_expr(*y);
                lhs.les(&rhs)
            }
            AstExpr::Geq(x, y) => {
                let lhs = self.execute_expr(*x);
                let rhs = self.execute_expr(*y);
                lhs.geq(&rhs)
            }
            AstExpr::Leq(x, y) => {
                let lhs = self.execute_expr(*x);
                let rhs = self.execute_expr(*y);
                lhs.leq(&rhs)
            }
            AstExpr::And(x, y) => {
                let lhs = self.execute_expr(*x);
                let rhs = self.execute_expr(*y);
                lhs.and(&rhs)
            }
            AstExpr::Or(x, y) => {
                let lhs = self.execute_expr(*x);
                let rhs = self.execute_expr(*y);
                lhs.or(&rhs)
            }
            AstExpr::Not(x) => {
                let x = self.execute_expr(*x);
                x.not()
            }
            AstExpr::Assign(n, v) => {
                let v = self.execute_expr(*v);
                self.environment.set_v(n, v.clone());
                v
            }
            AstExpr::FnCall(mem, args) => {
                let caller = self.execute_expr(*mem);
                match caller {
                    TiObj::Fn(fdef, fbody, environment) => {
                        let mut vm = TiVM::with_environment(self.environment.clone());
                        vm.environment.with(environment);
                        let args: Vec<TiObj> = args
                            .into_iter()
                            .map(|expr| self.execute_expr(expr))
                            .collect();
                        for idx in 0..args.len() {
                            vm.environment
                                .set(fdef.arguments[idx].name.clone(), args[idx].clone());
                        }
                        vm.execute_block(fbody)
                    }
                    _ => {
                        build_ti_error!(@err "type error: type `{:?}` is not callable.", caller.as_type())
                    }
                }
            }
            AstExpr::Var(x) => {
                if let Some(res) = self.environment.get(&x) {
                    res
                } else {
                    build_ti_error!(@err "reference error: name `{}` is not declared or has already moved.", x)
                }
            }
            AstExpr::LiteralNum(x) => TiObj::Num(x),
            AstExpr::LiteralStr(x) => TiObj::Str(x),
            AstExpr::LiteralUnit(x) => {
                TiObj::List(x.into_iter().map(|expr| self.execute_expr(expr)).collect())
            }
            AstExpr::LiteralArray(x) => {
                TiObj::List(x.into_iter().map(|expr| self.execute_expr(expr)).collect())
            }
            AstExpr::Neg(x) => {
                let res = self.execute_expr(*x);
                res.neg()
            }
            AstExpr::IfElse(cond, true_case, false_case) => {
                let cond = self.execute_expr(*cond);
                let cond = match &cond {
                    TiObj::Bool(x) => *x,
                    _ => {
                        build_ti_error!(@err "type error: type `{:?}` cannot be hidden cast where there's an if expression.", cond.as_type())
                    }
                };
                if cond {
                    self.execute_block(true_case)
                } else {
                    self.execute_block(false_case)
                }
            }
            AstExpr::Block(block) => self.execute_block(block),
            AstExpr::While(cond, body) => {
                while {
                    let cond = self.execute_expr(*cond.clone());
                    let cond = match &cond {
                        TiObj::Bool(x) => *x,
                        _ => {
                            build_ti_error!(@err "type error: type `{:?}` cannot be hidden cast where there's an if expression.", cond.as_type())
                        }
                    };
                    cond
                } {
                    self.execute_block(body.clone());
                }
                TiObj::build_unit()
            }
        }
    }

    fn execute_ast(&mut self, ast_node: AstNode) -> TiObj {
        match ast_node {
            AstNode::Program(program) => {
                for stmt in program.program {
                    self.execute_ast(stmt);
                }
                TiObj::build_unit()
            }
            AstNode::Fn(fdef, fbody) => {
                let name = fdef.name.clone();
                let ti_fn = TiObj::Fn(fdef, fbody, self.environment.clone());
                self.environment.set(name, ti_fn.clone());
                ti_fn
            }
            AstNode::Impl(_, _) => unreachable!(),
            AstNode::Let(name, value) => {
                let res = match value {
                    Some(v) => self.execute_expr(v),
                    None => TiObj::build_unit(),
                };
                self.environment.set(name, res);
                TiObj::build_unit()
            }
            AstNode::Expr(expr) => self.execute_expr(expr),
            AstNode::Empty => unreachable!(),
        }
    }

    pub fn execute(&mut self, ast_node: AstNode) -> TiObj {
        self.execute_ast(ast_node)
    }

    pub fn run_fn(&mut self, entry: &str) -> TiObj {
        if let Some(TiObj::Fn(_, entry_body, _)) =
            self.environment.move_out(&Rc::new(entry.to_string()))
        {
            self.execute_block(entry_body)
        } else {
            build_ti_error!(@err "runtime error: no entry.")
        }
    }
}
