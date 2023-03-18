use std::{rc::Rc, collections::HashMap};

#[derive(Debug)]
pub enum Type<'a> {
  Unknown,
  Never,
  F32,
  F64,
  I8,
  I16,
  I32,
  I64,
  I128,
  U8,
  U16,
  U32,
  U64,
  U128,
  Bool,
  Ref(Box<Type<'a>>),
  Array(Box<Type<'a>>, usize),
  Unit(Vec<Type<'a>>),
  // Impled(&'a TraitDef<'a>),
  // Struct(&'a StructDef<'a>),
  // Enum(&'a EnumDef<'a>),
  _NeverUseful(&'a str), // this is used for saving the lifetime 'a
  Costume(Rc<String>),
  Anna(Box<Type<'a>>, Vec<Type<'a>>),
}

#[derive(Debug)]
pub struct FnArg {
  pub name: Rc<String>,
  pub a_type: Rc<String>,
}

impl FnArg {
  pub fn new(name: Rc<String>, a_type: Rc<String>) -> Self {
    Self { name, a_type }
  }
}

#[derive(Debug)]
pub struct FnDef<'a> {
  pub name: Rc<String>,
  pub types: HashMap<Rc<String>, Type<'a>>,
  pub arguments: Vec<FnArg>,
  pub ret_type: Rc<String>,
}

#[derive(Debug)]
pub struct StructField {
  pub name: Rc<String>,
  pub a_type: Rc<String>,
}

#[derive(Debug)]
pub struct StructDef<'a> {
  pub name: Rc<String>,
  pub types: HashMap<Rc<String>, Type<'a>>,
  pub fields: Vec<StructField>,
}

#[derive(Debug)]
pub struct EnumField {
  pub name: Rc<String>,
  pub values: Vec<Rc<String>>,
}

#[derive(Debug)]
pub struct EnumDef<'a> {
  pub name: Rc<String>,
  pub types: HashMap<Rc<String>, Type<'a>>,
  pub fields: Vec<EnumField>,
}

#[derive(Debug)]
pub struct TraitField<'a> {
  pub body: FnDef<'a>,
}

#[derive(Debug)]
pub struct TraitDef<'a> {
  pub name: Rc<String>,
  pub fields: Vec<TraitField<'a>>,
}

#[derive(Debug)]
pub struct VarDef<'a> {
  pub name: Rc<String>,
  pub v_type: Type<'a>,
}

#[derive(Debug, Clone)]
pub enum Scope {
  Global,
  Block(usize, usize), // from token# to token#
}

pub enum ScopeOrdering {
  Contains,
  Cross,
  RContains
}

impl Scope {
  pub fn order(&self, other: &Self) -> ScopeOrdering {
    match self {
      Scope::Global => ScopeOrdering::Contains,
      Scope::Block(lf, lt) => match other {
        Scope::Global => ScopeOrdering::RContains,
        Scope::Block(rf, rt) => {
          if rf > lf && rt < lt { // lf  rf  rt  lt
            ScopeOrdering::Contains
          } else if rf < lf && rt > lt { // rf  lf  lt  rt
            ScopeOrdering::RContains
          } else {
            ScopeOrdering::Cross
          }
        }
      }
    }
  }
}

#[derive(Debug)]
pub struct WithScope<T> {
  pub scope: Scope,
  pub body: T,
}

impl<T> WithScope<T> {
  pub fn global(body: T) -> Self {
    Self { 
      body, scope: Scope::Global,
    }
  }
}

#[derive(Debug)]
pub enum AstNode {
  Program(AstProgram),
  Fn(Rc<String>, AstBlock),
  Impl(Rc<String>, Vec<AstNode>),
  Let(Rc<String>, AstExpr),
  Expr(AstExpr),
  // Block(AstBlock),
  Empty,
}

#[derive(Debug)]
pub struct AstBlock {
  pub block: Vec<AstNode>,
}

impl AstBlock {
  pub fn new() -> Self {
    Self {
      block: Vec::new(),
    }
  }

  pub fn add(&mut self, ast_node: AstNode) {
    self.block.push(ast_node);
  }
}

#[derive(Debug)]
pub struct AstProgram {
  pub program: Vec<AstNode>,
}

impl AstProgram {
  pub fn new() -> Self {
    Self {
      program: Vec::new(),
    }
  }

  pub fn add(&mut self, ast_node: AstNode) {
    self.program.push(ast_node);
  }
}

#[derive(Debug)]
pub enum AstExpr {
  Add(Box<AstExpr>, Box<AstExpr>),
  Sub(Box<AstExpr>, Box<AstExpr>),
  Mul(Box<AstExpr>, Box<AstExpr>),
  Div(Box<AstExpr>, Box<AstExpr>),
  AddEq(Box<AstExpr>, Box<AstExpr>),
  SubEq(Box<AstExpr>, Box<AstExpr>),
  MulEq(Box<AstExpr>, Box<AstExpr>),
  DivEq(Box<AstExpr>, Box<AstExpr>),
  Eq(Box<AstExpr>, Box<AstExpr>),
  Neq(Box<AstExpr>, Box<AstExpr>),
  Grt(Box<AstExpr>, Box<AstExpr>),
  Les(Box<AstExpr>, Box<AstExpr>),
  Geq(Box<AstExpr>, Box<AstExpr>),
  Leq(Box<AstExpr>, Box<AstExpr>),
  Not(Box<AstExpr>, Box<AstExpr>),
  Assign(Box<AstExpr>, Box<AstExpr>),
  FnCall(Box<AstExpr>, Vec<AstExpr>),
  Var(Rc<String>),

  LiteralNum(f64),
  LiteralStr(String),
  LiteralUnit(Vec<AstExpr>),
  LiteralArray(Vec<AstExpr>),
}