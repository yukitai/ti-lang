use std::collections::HashMap;

use crate::{build_ti_error, vm::environment::environment::TiEnvironment, frontend::parser::ast::{FnDef, AstBlock}};

macro_rules! impl_binary_operator {
  (@full $op_num: tt, $op_str: tt, $op_bool: tt, $a: expr, $b: expr) => {
    match $a {
      TiObj::Map(ref x) => match $b {
        TiObj::Map(y) => {
          todo!()
        },
        _ => {
          build_ti_error!(@err "type error: mismatched type `{:?}` and `{:?}`.", $a.as_type(), $b.as_type())
        },
      },
      TiObj::List(ref x) => match $b {
        TiObj::List(y) => {
          todo!()
        },
        _ => {
          build_ti_error!(@err "type error: mismatched type `{:?}` and `{:?}`.", $a.as_type(), $b.as_type())
        },
      },
      TiObj::Num(x) => match $b {
        TiObj::Num(y) => {
          TiObj::Num(x $op_num y)
        },
        _ => {
          build_ti_error!(@err "type error: mismatched type `{:?}` and `{:?}`.", $a.as_type(), $b.as_type())
        },
      },
      TiObj::Str(ref x) => match $b {
        TiObj::Str(y) => {
          TiObj::Str(x.clone() $op_str y)
        },
        _ => {
          build_ti_error!(@err "type error: mismatched type `{:?}` and `{:?}`.", $a.as_type(), $b.as_type())
        },
      },
      TiObj::Bool(x) => match $b {
        TiObj::Bool(y) => {
          TiObj::Bool(x $op_bool *y)
        },
        _ => {
          build_ti_error!(@err "type error: mismatched type `{:?}` and `{:?}`.", $a.as_type(), $b.as_type())
        },
      },
      TiObj::Fn(_, _, _) => {
        build_ti_error!(@err "type error: the operator `{}` is not implemtnted for type `{:?}`.", stringify!($op_num), TiObjType::Fn)
      },
    }
  };

  (@num $op_num: tt, $a: expr, $b: expr) => {
    match $a {
      TiObj::Map(ref x) => match $b {
        TiObj::Map(y) => {
          todo!()
        },
        _ => {
          build_ti_error!(@err "type error: mismatched type `{:?}` and `{:?}`.", $a.as_type(), $b.as_type())
        },
      },
      TiObj::List(ref x) => match $b {
        TiObj::List(y) => {
          todo!()
        },
        _ => {
          build_ti_error!(@err "type error: mismatched type `{:?}` and `{:?}`.", $a.as_type(), $b.as_type())
        },
      },
      TiObj::Num(x) => match $b {
        TiObj::Num(y) => {
          TiObj::Num(x $op_num y)
        },
        _ => {
          build_ti_error!(@err "type error: mismatched type `{:?}` and `{:?}`.", $a.as_type(), $b.as_type())
        },
      },
      TiObj::Str(_) => {
        build_ti_error!(@err "type error: the operator `{}` is not implemtnted for type `{:?}`.", stringify!($op_num), TiObjType::Str)
      },
      TiObj::Bool(_) => {
        build_ti_error!(@err "type error: the operator `{}` is not implemtnted for type `{:?}`.", stringify!($op_num), TiObjType::Bool)
      },
      TiObj::Fn(_, _, _) => {
        build_ti_error!(@err "type error: the operator `{}` is not implemtnted for type `{:?}`.", stringify!($op_num), TiObjType::Fn)
      },
    }
  };
}

#[derive(Debug, Clone)]
pub enum TiObj {
  Map(HashMap<TiObj, TiObj>),
  List(Vec<TiObj>),
  Num(f64),
  Str(String),
  Bool(bool),
  Fn(FnDef, AstBlock, TiEnvironment)
}

#[derive(Debug)]
pub enum TiObjType {
  Map,
  List,
  Num,
  Str,
  Bool,
  Fn,
}

impl TiObj {
  pub fn as_type(&self) -> TiObjType {
    match self {
      TiObj::Map(_) => TiObjType::Map,
      TiObj::List(_) => TiObjType::List,
      TiObj::Num(_) => TiObjType::Num,
      TiObj::Str(_) => TiObjType::Str,
      TiObj::Bool(_) => TiObjType::Bool,
      TiObj::Fn(_, _, _) => TiObjType::Fn,
    }
  }

  pub fn add(self, b: &TiObj) -> TiObj {
    impl_binary_operator!(@full +, +, ||, self, b)
  }

  pub fn sub(self, b: &TiObj) -> TiObj {
    impl_binary_operator!(@num -, self, b)
  }

  pub fn mul(self, b: &TiObj) -> TiObj {
    impl_binary_operator!(@num *, self, b)
  }

  pub fn div(self, b: &TiObj) -> TiObj {
    impl_binary_operator!(@num /, self, b)
  }

  pub fn r#mod(self, b: &TiObj) -> TiObj {
    impl_binary_operator!(@num %, self, b)
  }
}

impl TiObj {
  pub fn BuildUnit() -> Self {
    TiObj::List(Vec::new())
  }
}