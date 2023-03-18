use std::{rc::Rc, collections::HashMap};

use crate::{frontend::lexer::token::{TokenStream, TokenType}, build_ti_error};

use super::ast::*;

#[derive(Debug)]
pub struct Parser<'a> {
  sym_id: usize,
  pub fn_def: Vec<WithScope<FnDef<'a>>>,
  pub struct_def: Vec<WithScope<StructDef<'a>>>,
  pub enum_def: Vec<WithScope<EnumDef<'a>>>,
  pub trait_def: Vec<WithScope<TraitDef<'a>>>,
  pub var_def: Vec<WithScope<VarDef<'a>>>,
  pub tokens: TokenStream,
  pub ast: AstNode,
}

impl<'a> Parser<'a> {
  pub fn new(tokens: TokenStream) -> Self {
    Self {
      sym_id: 0,
      fn_def: Vec::new(),
      struct_def: Vec::new(),
      enum_def: Vec::new(),
      trait_def: Vec::new(),
      var_def: Vec::new(),
      tokens,
      ast: AstNode::Empty,
    }
  }
}

impl<'a> Parser<'a> {
  pub fn parse(&mut self) {
    let mut program = AstProgram::new();
    while !self.tokens.is_eof() {
      if !self.tokens.assert_next_tier(0) { build_ti_error!(@at self.tokens.peek(), @err "Unexpected Token `{:?}`", self.tokens.peek()) }
      if self.tokens.is_eof() { break }
      if let Some(ast_node) = self.parse_definion(0, Scope::Global) {
        if let Some(ast_node) = ast_node {
          program.add(ast_node);
        }
      } else {
        self.tokens.forward();
        // println!("{:#?}", self);
        // panic!("Unexpeced Token `{:?}`.", self.tokens.next())
      }
    }
    self.ast = AstNode::Program(program);
  }

  fn sym_name(&mut self) -> Rc<String> {
    self.sym_id += 1;
    Rc::new(format!("${}", self.sym_id))
  }

  fn parse_definion(&mut self, tier: usize, scope: Scope) -> Option<Option<AstNode>> {
    let token = self.tokens.next();
    match token.t_type {
      TokenType::KeywordFn => {
        Some(self.parse_fn_definion(tier, scope))
      },
      TokenType::KeywordEnum => {
        self.parse_enum_definion(tier, scope);
        Some(None)
      },
      TokenType::KeywordStruct => {
        self.parse_struct_definion(tier, scope);
        Some(None)
      },
      TokenType::KeywordTrait => {
        self.parse_trait_definion(tier, scope);
        Some(None)
      },
      TokenType::KeywordImpl => {
        Some(self.parse_impl_definion(tier, scope))
      },
      _ => {
        self.tokens.backward();
        self.tokens.backward();
        None
      },
    }
  }

  fn parse_fn_definion(&mut self, tier: usize, _scope: Scope) -> Option<AstNode> {
    let mut types = HashMap::new();
    let fname;
    let mut fargs: Vec<FnArg> = Vec::new();
    let ret_t: Rc<String>;
    match self.tokens.next().t_type.clone() {
      TokenType::OperatorLes => {
        // fn<TN: TT[, ...]> FN(FA: FT[, ...])
        todo!()
      },
      TokenType::Identifier(n) => {
        // fn FN(FA: FT[, ...])
        fname = n;
      },
      _ => build_ti_error!(@at self.tokens.peek(), @err "Unexpect Token `{:?}`", self.tokens.peek())
    }
    if !self.tokens.assert_next(TokenType::OpenParen) {
      build_ti_error!(@at self.tokens.peek(), @err "Expect Token `(`, found `{:?}`", self.tokens.peek())
    }
    loop {
      if self.tokens.assert_next(TokenType::CloseParen) {
        break
      }
      if let TokenType::Identifier(argn) = self.tokens.next().t_type.clone() {
        if !self.tokens.assert_next(TokenType::OperatorColon) {
          build_ti_error!(@at self.tokens.peek(), @err "Expect Token `:`, found {:?}", self.tokens.peek())
        }
        let argt = self.parse_type();
        let a = self.sym_name();
        types.insert(a.clone(), argt);
        let arg = FnArg::new(argn.clone(), a);
        fargs.push(arg);
      } else {
        build_ti_error!(@at self.tokens.last(), @err "Expect Identifier or Token `)`, found {:?}", self.tokens.last())
      }
      if self.tokens.assert_next(TokenType::CloseParen) {
        break
      }
      self.tokens.forward();
      // self.tokens.forward();
      if self.tokens.is_eof() {
        build_ti_error!(@err "Expect Token `)`, found `Eof`")
      }
    }

    let ret_type = if self.tokens.assert_next(TokenType::OperatorArrow) {
      self.parse_type()
    } else {
      Type::Unit(Vec::new())
    };
    let a = self.sym_name();
    types.insert(a.clone(), ret_type);
    ret_t = a;
    self.fn_def.push(WithScope::global(FnDef {
      name: fname.clone(),
      types,
      arguments: fargs,
      ret_type: ret_t,
    }));

    if self.tokens.assert_next(TokenType::OperatorAssign) {
      let mut fbody;
      if self.tokens.assert_next_tier(tier + 1) {
        fbody = self.parse_block(tier + 1);
      } else {
        let expr = self.parse_expr();
        fbody = AstBlock::new();
        fbody.add(AstNode::Expr(expr));
      }
      Some(AstNode::Fn(fname, fbody))
    } else {
      None
    }
  }

  fn parse_enum_definion(&mut self, tier: usize, scope: Scope) {
    todo!()
  }

  fn parse_struct_definion(&mut self, tier: usize, scope: Scope) {
    todo!()
  }

  fn parse_trait_definion(&mut self, tier: usize, scope: Scope) {
    todo!()
  }

  fn parse_impl_definion(&mut self, tier: usize, scope: Scope) -> Option<AstNode> {
    todo!()
  }
  
  fn parse_expr(&mut self) -> AstExpr {
    self.parse_expr_add()
  }

  fn parse_expr_primary(&mut self) -> AstExpr {
    let curr = self.tokens.next();
    match &curr.t_type {
      TokenType::Identifier(x) => AstExpr::Var(x.clone()),
      TokenType::LiteralNum(x) => AstExpr::LiteralNum(*x),
      TokenType::LiteralStr(x) => AstExpr::LiteralStr(x.clone()),
      TokenType::OpenParen => {
        let expr = self.parse_expr();
        if !self.tokens.assert_next(TokenType::CloseParen) {
          build_ti_error!(@at self.tokens.peek(), @err "Expect Token `)`, found {:?}.", self.tokens.peek())
        }
        expr
      },
      _ => {
        build_ti_error!(@at curr, @err "Unexpected Token `{:?}` While Parsing an Expr.", curr)
      },
    }
  }

  fn parse_expr_add(&mut self) -> AstExpr {
    let mut lhs = self.parse_expr_mul();
    while !self.tokens.is_eof() {
      let curr = self.tokens.peek();
      match curr.t_type {
        TokenType::OperatorAdd => {
          self.tokens.forward();
          let rhs = self.parse_expr_mul();
          lhs = AstExpr::Add(Box::new(lhs), Box::new(rhs));
        },
        TokenType::OperatorSub => {
          self.tokens.forward();
          let rhs = self.parse_expr_mul();
          lhs = AstExpr::Sub(Box::new(lhs), Box::new(rhs));
        },
        _ => break,
      }
    }
    lhs
  }

  fn parse_expr_mul(&mut self) -> AstExpr {
    let mut lhs = self.parse_expr_call();
    while !self.tokens.is_eof() {
      let curr = self.tokens.peek();
      match curr.t_type {
        TokenType::OperatorMul => {
          self.tokens.forward();
          let rhs = self.parse_expr_call();
          lhs = AstExpr::Mul(Box::new(lhs), Box::new(rhs));
        },
        TokenType::OperatorDiv => {
          self.tokens.forward();
          let rhs = self.parse_expr_call();
          lhs = AstExpr::Div(Box::new(lhs), Box::new(rhs));
        },
        _ => break,
      }
    }
    lhs
  }

  fn parse_expr_call(&mut self) -> AstExpr {
    let member = self.parse_expr_primary();
    if self.tokens.assert_next(TokenType::OpenParen) {
      let mut args = Vec::new();
      loop {
        let expr = self.parse_expr();
        args.push(expr);
        if self.tokens.assert_next(TokenType::CloseParen) {
          break
        }
        self.tokens.forward();
      }
      AstExpr::FnCall(Box::new(member), args)
    } else {
      member
    }
  }

  fn find_scope(&self, tier: usize) -> Scope {
    let mut idx = self.tokens.curr;
    while !self.tokens.is_eof() {
      match self.tokens.at(idx).unwrap().t_type {
        TokenType::IdentTier(x) => {
          if x < tier { break }
        }
        _ => {},
      }
      idx += 1;
    }
    Scope::Block(self.tokens.curr, idx)
  }
  
  fn parse_block(&mut self, tier: usize) -> AstBlock {
    let mut block = AstBlock::new();
    let scope = self.find_scope(tier);
    self.tokens.backward();
    while !self.tokens.is_eof() {
      if let Some(ast_node) = self.parse_stmt(tier, &scope) {
        block.add(ast_node);
      } else {
        break
      }
    }
    block
  }

  fn parse_stmt(&mut self, tier: usize, scope: &Scope) -> Option<AstNode> {
    if self.tokens.assert_next_tier(tier) {
      if let Some(ast_node) = self.parse_definion(tier, scope.clone()) {
        Some(ast_node.unwrap_or(AstNode::Empty))
      } else {
        self.tokens.forward();
        let expr = self.parse_expr();
        Some(AstNode::Expr(expr))
      }
    } else {
      self.tokens.forward();
      None
    }
  }

  fn parse_type(&mut self) -> Type<'a> {
    self.parse_type_anna()
  }

  fn parse_type_anna(&mut self) -> Type<'a> {
    let t = self.parse_type_primary();
    if self.tokens.assert_next(TokenType::OperatorLes) {
      let mut anna = Vec::new();
      loop {
        let expr = self.parse_type();
        anna.push(expr);
        if self.tokens.assert_next(TokenType::OperatorGrt) {
          break
        }
        self.tokens.forward();
      }
      Type::Anna(Box::new(t), anna)
    } else {
      t
    }
  }

  fn parse_type_primary(&mut self) -> Type<'a> {
    let curr = self.tokens.next();
    match &curr.t_type {
      TokenType::Identifier(t) => {
        match t.as_str() {
          "i8" => Type::I8,
          "i16" => Type::I16,
          "i32" => Type::I32,
          "i64" => Type::I64,
          "i128" => Type::I128,
          "u8" => Type::U8,
          "u16" => Type::U16,
          "u32" => Type::U32,
          "u64" => Type::U64,
          "u128" => Type::U128,
          "bool" => Type::Bool,
          "f32" => Type::F32,
          "f64" => Type::F64,
          _ => {
            Type::Costume(t.clone())
          },
        }
      },
      TokenType::OpenParen => {
        let mut unit = Vec::new();
        loop {
          let expr = self.parse_type();
          unit.push(expr);
          if self.tokens.assert_next(TokenType::CloseParen) {
            break
          }
          self.tokens.forward();
        }
        Type::Unit(unit)
      },
      TokenType::OpenBrace => {
        let t = self.parse_type();
        let l = if self.tokens.assert_next(TokenType::Semi) {
          if let TokenType::LiteralNum(l) = self.tokens.next().t_type {
            l as usize
          } else {
            build_ti_error!(@at self.tokens.peek(), @err "Expect an Literal Number, found {:?}", self.tokens.peek())
          }
        } else {
          0
        };
        if !self.tokens.assert_next(TokenType::CloseBrace) {
          build_ti_error!(@at self.tokens.peek(), @err "Expect Token `]`, found {:?}", self.tokens.peek())
        }
        Type::Array(Box::new(t), l)
      },
      TokenType::OperatorRef => {
        let t = self.parse_type();
        Type::Ref(Box::new(t))
      }
      _ => {
        build_ti_error!(@at curr, @err "Expect Identifier, found {:?}", curr)
      },
    }
  }
}