use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
  LiteralNum(f64),
  LiteralStr(String),
  LiteralBool(bool),
  Identifier(Rc<String>),
  IdentTier(usize),
  OperatorAssign,
  OperatorAdd,
  OperatorSub,
  OperatorMul,
  OperatorDiv,
  OperatorAddAssign,
  OperatorSubAssign,
  OperatorMulAssign,
  OperatorDivAssign,
  OperatorEq,
  OperatorNeq,
  OperatorLes,
  OperatorGrt,
  OperatorLeq,
  OperatorGeq,
  OperatorAnd,
  OperatorOr,
  OperatorNot,
  OperatorRef,
  OperatorArrow,
  OperatorFatArrow,
  OperatorComma,
  OperatorColon,
  /* OperatorDeref,*/ // Equals to OperatorMul
  KeywordLet,
  KeywordFn,
  KeywordTrait,
  KeywordEnum,
  KeywordStruct,
  KeywordConst,
  KeywordImpl,
  OpenParen,
  CloseParen,
  OpenBrace,
  CloseBrace,
  Semi,
}

pub type TokenAt = (usize, (usize, usize));

#[derive(Debug, Clone)]
pub struct Token {
  pub t_at: TokenAt,
  pub t_type: TokenType,
}

impl Token {
  pub fn new(t_type: TokenType, t_at: TokenAt) -> Self {
    Self {
      t_at, t_type, 
    }
  }
}

#[derive(Debug)]
pub struct TokenStream {
  tokens: Vec<Token>,
  pub curr: usize,
}

impl TokenStream {
  pub fn new(tokens: Vec<Token>) -> Self {
    Self {
      tokens, curr: 0,
    }
  }

  #[inline]
  pub fn forward(&mut self) {
    self.curr += 1;
  }
  #[inline]
  pub fn backward(&mut self) {
    self.curr -= 1;
  }
  #[inline]
  pub fn next(&mut self) -> &Token {
    let next = self.tokens.get(self.curr).unwrap();
    self.curr += 1;
    next
  }
  #[inline]
  pub fn peek(&self) -> &Token {
    self.tokens.get(self.curr).unwrap()
  }
  #[inline]
  pub fn last(&self) -> &Token {
    self.tokens.get(self.curr - 1).unwrap()
  }
  #[inline]
  pub fn is_eof(&self) -> bool {
    self.curr >= self.tokens.len()
  }
  
  #[inline]
  pub fn at(&self, idx: usize) -> Option<&Token> {
    self.tokens.get(idx)
  }

  #[inline]
  pub fn assert_next_ident(&mut self) -> bool {
    if let TokenType::Identifier(_) = self.tokens.get(self.curr).unwrap().t_type {
      self.forward();
      true
    } else { false }
  }

  #[inline]
  pub fn assert_next_tier(&mut self, tier: usize) -> bool {
    if let TokenType::IdentTier(x) = self.tokens.get(self.curr).unwrap().t_type {
      if x == tier {
        self.forward();
        true
      } else { false }
    } else { false }
  }

  #[inline]
  pub fn assert_next(&mut self, t_type: TokenType) -> bool {
    let res = self.tokens.get(self.curr).unwrap().t_type == t_type;
    if res {
      self.forward();
      true
    } else { false }
  }
}