use std::collections::HashMap;

use crate::{
    build_ti_error,
    frontend::parser::ast::{AstBlock, FnDef},
    vm::environment::environment::TiEnvironment,
};

macro_rules! impl_binary_operator {
  (@num $op_num: tt $num_t: ident $($num_b: tt)? @str $op_str: tt $str_t: ident @bool $op_bool: tt $bool_t: ident $($bool_b: tt)?, $a: expr, $b: expr) => {
    match $a {
      TiObj::Map(_) => match $b {
        TiObj::Map(_) => {
          todo!()
        },
        _ => {
          build_ti_error!(@err "type error: mismatched type `{:?}` and `{:?}`.", $a.as_type(), $b.as_type())
        },
      },
      TiObj::List(_) => match $b {
        TiObj::List(_) => {
          todo!()
        },
        _ => {
          build_ti_error!(@err "type error: mismatched type `{:?}` and `{:?}`.", $a.as_type(), $b.as_type())
        },
      },
      TiObj::Num(x) => match $b {
        TiObj::Num(y) => {
          TiObj::$num_t(x $op_num $($num_b)? y)
        },
        _ => {
          build_ti_error!(@err "type error: mismatched type `{:?}` and `{:?}`.", $a.as_type(), $b.as_type())
        },
      },
      TiObj::Str(ref x) => match $b {
        TiObj::Str(y) => {
          TiObj::$str_t(x $op_str y)
        },
        _ => {
          build_ti_error!(@err "type error: mismatched type `{:?}` and `{:?}`.", $a.as_type(), $b.as_type())
        },
      },
      TiObj::Bool(x) => match $b {
        TiObj::Bool(y) => {
          TiObj::$bool_t(x $op_bool $($bool_b)? y)
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

  (@num $op_num: tt $num_t: ident $($num_b: tt)? @str $op_str: tt $str_t: ident, $a: expr, $b: expr) => {
    match $a {
      TiObj::Map(_) => match $b {
        TiObj::Map(_) => {
          todo!()
        },
        _ => {
          build_ti_error!(@err "type error: mismatched type `{:?}` and `{:?}`.", $a.as_type(), $b.as_type())
        },
      },
      TiObj::List(_) => match $b {
        TiObj::List(_) => {
          todo!()
        },
        _ => {
          build_ti_error!(@err "type error: mismatched type `{:?}` and `{:?}`.", $a.as_type(), $b.as_type())
        },
      },
      TiObj::Num(x) => match $b {
        TiObj::Num(y) => {
          TiObj::$num_t(x $op_num $($num_b)? y)
        },
        _ => {
          build_ti_error!(@err "type error: mismatched type `{:?}` and `{:?}`.", $a.as_type(), $b.as_type())
        },
      },
      TiObj::Str(ref x) => match $b {
        TiObj::Str(y) => {
          TiObj::$str_t(x $op_str y)
        },
        _ => {
          build_ti_error!(@err "type error: mismatched type `{:?}` and `{:?}`.", $a.as_type(), $b.as_type())
        },
      },
      TiObj::Bool(_) => {
        build_ti_error!(@err "type error: the operator `{}` is not implemtnted for type `{:?}`.", stringify!($op_num), TiObjType::Fn)
      },
      TiObj::Fn(_, _, _) => {
        build_ti_error!(@err "type error: the operator `{}` is not implemtnted for type `{:?}`.", stringify!($op_num), TiObjType::Fn)
      },
    }
  };

  (@num $op_num: tt $num_t: ident $($num_b: tt)? @str_owned $op_str: tt $str_t: ident, $a: expr, $b: expr) => {
    match $a {
      TiObj::Map(_) => match $b {
        TiObj::Map(_) => {
          todo!()
        },
        _ => {
          build_ti_error!(@err "type error: mismatched type `{:?}` and `{:?}`.", $a.as_type(), $b.as_type())
        },
      },
      TiObj::List(_) => match $b {
        TiObj::List(_) => {
          todo!()
        },
        _ => {
          build_ti_error!(@err "type error: mismatched type `{:?}` and `{:?}`.", $a.as_type(), $b.as_type())
        },
      },
      TiObj::Num(x) => match $b {
        TiObj::Num(y) => {
          TiObj::$num_t(x $op_num $($num_b)? y)
        },
        _ => {
          build_ti_error!(@err "type error: mismatched type `{:?}` and `{:?}`.", $a.as_type(), $b.as_type())
        },
      },
      TiObj::Str(ref x) => match $b {
        TiObj::Str(y) => {
          TiObj::$str_t(x.to_owned() $op_str y)
        },
        _ => {
          build_ti_error!(@err "type error: mismatched type `{:?}` and `{:?}`.", $a.as_type(), $b.as_type())
        },
      },
      TiObj::Bool(_) => {
        build_ti_error!(@err "type error: the operator `{}` is not implemtnted for type `{:?}`.", stringify!($op_num), TiObjType::Fn)
      },
      TiObj::Fn(_, _, _) => {
        build_ti_error!(@err "type error: the operator `{}` is not implemtnted for type `{:?}`.", stringify!($op_num), TiObjType::Fn)
      },
    }
  };

  (@num $op_num: tt $num_t: ident, $a: expr, $b: expr) => {
    match $a {
      TiObj::Map(_) => match $b {
        TiObj::Map(_) => {
          todo!()
        },
        _ => {
          build_ti_error!(@err "type error: mismatched type `{:?}` and `{:?}`.", $a.as_type(), $b.as_type())
        },
      },
      TiObj::List(_) => match $b {
        TiObj::List(_) => {
          todo!()
        },
        _ => {
          build_ti_error!(@err "type error: mismatched type `{:?}` and `{:?}`.", $a.as_type(), $b.as_type())
        },
      },
      TiObj::Num(x) => match $b {
        TiObj::Num(y) => {
          TiObj::$num_t(x $op_num y)
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

  (@bool $op_bool: tt $bool_t: ident, $a: expr, $b: expr) => {
    match $a {
      TiObj::Map(_) => match $b {
        TiObj::Map(_) => {
          todo!()
        },
        _ => {
          build_ti_error!(@err "type error: mismatched type `{:?}` and `{:?}`.", $a.as_type(), $b.as_type())
        },
      },
      TiObj::List(_) => match $b {
        TiObj::List(_) => {
          todo!()
        },
        _ => {
          build_ti_error!(@err "type error: mismatched type `{:?}` and `{:?}`.", $a.as_type(), $b.as_type())
        },
      },
      TiObj::Num(_) => {
        build_ti_error!(@err "type error: mismatched type `{:?}` and `{:?}`.", $a.as_type(), $b.as_type())
      },
      TiObj::Str(_) => {
        build_ti_error!(@err "type error: the operator `{}` is not implemtnted for type `{:?}`.", stringify!($op_num), TiObjType::Str)
      },
      TiObj::Bool(x) => match $b {
        TiObj::Bool(y) => {
          TiObj::$bool_t(x $op_bool y)
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
}

#[derive(Debug, Clone)]
pub enum TiObj {
    Map(HashMap<TiObj, TiObj>),
    List(Vec<TiObj>),
    Num(f64),
    Str(String),
    Bool(bool),
    Fn(FnDef, AstBlock, TiEnvironment),
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
        impl_binary_operator!(@num + Num * @str_owned + Str, self, b)
    }

    pub fn sub(self, b: &TiObj) -> TiObj {
        impl_binary_operator!(@num - Num, self, b)
    }

    pub fn mul(self, b: &TiObj) -> TiObj {
        impl_binary_operator!(@num * Num, self, b)
    }

    pub fn div(self, b: &TiObj) -> TiObj {
        impl_binary_operator!(@num / Num, self, b)
    }

    pub fn r#mod(self, b: &TiObj) -> TiObj {
        impl_binary_operator!(@num % Num, self, b)
    }

    pub fn neg(self) -> Self {
        todo!()
    }

    pub fn not(self) -> Self {
        todo!()
    }

    pub fn les(self, b: &TiObj) -> Self {
        impl_binary_operator!(@num < Bool * @str < Bool, self, b)
    }

    pub fn grt(self, b: &TiObj) -> Self {
        impl_binary_operator!(@num > Bool * @str > Bool, self, b)
    }

    pub fn eq(self, b: &TiObj) -> Self {
        impl_binary_operator!(@num == Bool * @str == Bool @bool == Bool *, self, b)
    }

    pub fn neq(self, b: &TiObj) -> Self {
        impl_binary_operator!(@num != Bool * @str != Bool @bool != Bool *, self, b)
    }

    pub fn leq(self, b: &TiObj) -> Self {
        impl_binary_operator!(@num <= Bool * @str <= Bool, self, b)
    }

    pub fn geq(self, b: &TiObj) -> Self {
        impl_binary_operator!(@num >= Bool * @str >= Bool, self, b)
    }

    pub fn and(self, b: &TiObj) -> Self {
        impl_binary_operator!(@bool && Bool, self, *b)
    }

    pub fn or(self, b: &TiObj) -> Self {
        impl_binary_operator!(@bool || Bool, self, *b)
    }
}

impl TiObj {
    pub fn build_unit() -> Self {
        TiObj::List(Vec::new())
    }
}
