use std::{string::FromUtf8Error, rc::Rc};

use super::token::{Token, TokenType, TokenAt, TokenStream};

static SPACE_SIZE: u8 = 2;

pub struct Lexer {
  src: Vec<char>,
  mark: usize,
  curr: usize,
  line: usize,
}

impl Lexer {
  pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, FromUtf8Error> {
    let src_str = String::from_utf8(bytes)?;
    let src: Vec<char> = src_str.chars().collect();
    Ok(Self {
      src, mark: 0, curr: 0, line: 1,
    })
  }
}

impl Lexer {
  #[inline]
  fn forward(&mut self) {
    self.curr += 1;
  }
  #[inline]
  fn backward(&mut self) {
    self.curr -= 1;
  }
  #[inline]
  fn next(&mut self) -> &char {
    let next = self.src.get(self.curr).unwrap();
    self.curr += 1;
    next
  }
  #[inline]
  fn peek(&self) -> &char {
    self.src.get(self.curr).unwrap()
  }
  #[inline]
  fn is_eof(&self) -> bool {
    self.curr >= self.src.len()
  }

  #[inline]
  fn here(&self) -> TokenAt {
    (self.line, (self.curr, self.curr + 1))
  }
  #[inline]
  fn mark(&mut self) {
    self.mark = self.curr;
  }
  #[inline]
  fn range(&mut self) -> TokenAt {
    (self.line, (self.mark, self.curr + 1))
  }

  pub fn tokenize(&mut self) -> TokenStream {
    let mut tokens = Vec::new();
    { // parse the first ident tier
      self.mark();
      let mut tabs = 0;
      let mut spaces = 0;
      while !self.is_eof() {
        let curr = self.next();
        match curr {
          '\n' | '\r' => {},
          '\t' => {
            tabs += 1;
          },
          ' ' => {
            spaces += 1;
          }
          _ => {
            self.backward();
            break
          },
        }
      }
      tokens.push(Token::new(TokenType::IdentTier(tabs + spaces / (SPACE_SIZE as usize)), self.range()));
    }

    let mut ignore_nl = false;
    while !self.is_eof() {
      match self.next() {
        '(' => {
          tokens.push(Token::new(TokenType::OpenParen, self.here()));
        },
        ')' => {
          tokens.push(Token::new(TokenType::CloseParen, self.here()));
        },
        '+' => {
          if self.peek() == &'=' {
            self.mark();
            self.forward();
            tokens.push(Token::new(TokenType::OperatorAddAssign, self.range()));
          } else {
            tokens.push(Token::new(TokenType::OperatorAdd, self.here()));
          }
        },
        '-' => {
          if self.peek() == &'=' {
            self.mark();
            self.forward();
            tokens.push(Token::new(TokenType::OperatorSubAssign, self.range()));
          } else if self.peek() == &'>' {
            self.mark();
            self.forward();
            tokens.push(Token::new(TokenType::OperatorArrow, self.range()));
          } else {
            tokens.push(Token::new(TokenType::OperatorSub, self.here()));
          }
        },
        '*' => {
          if self.peek() == &'=' {
            self.mark();
            self.forward();
            tokens.push(Token::new(TokenType::OperatorMulAssign, self.range()));
          } else {
            tokens.push(Token::new(TokenType::OperatorMul, self.here()));
          }
        },
        '/' => {
          if self.peek() == &'=' {
            self.mark();
            self.forward();
            tokens.push(Token::new(TokenType::OperatorDivAssign, self.range()));
          } else {
            tokens.push(Token::new(TokenType::OperatorDiv, self.here()));
          }
        },
        '=' => {
          if self.peek() == &'=' {
            self.mark();
            self.forward();
            tokens.push(Token::new(TokenType::OperatorEq, self.range()));
          } else if self.peek() == &'>' {
            self.mark();
            self.forward();
            tokens.push(Token::new(TokenType::OperatorFatArrow, self.range()));
          } else {
            tokens.push(Token::new(TokenType::OperatorAssign, self.here()));
          }
        },
        '!' => {
          if self.peek() == &'=' {
            self.mark();
            self.forward();
            tokens.push(Token::new(TokenType::OperatorNeq, self.range()));
          } else {
            tokens.push(Token::new(TokenType::OperatorNot, self.here()));
          }
        },
        '<' => {
          if self.peek() == &'=' {
            self.mark();
            self.forward();
            tokens.push(Token::new(TokenType::OperatorLeq, self.range()));
          } else {
            tokens.push(Token::new(TokenType::OperatorLes, self.here()));
          }
        },
        '>' => {
          if self.peek() == &'=' {
            self.mark();
            self.forward();
            tokens.push(Token::new(TokenType::OperatorGeq, self.range()));
          } else {
            tokens.push(Token::new(TokenType::OperatorGrt, self.here()));
          }
        },
        '&' => {
          if self.peek() == &'&' {
            self.mark();
            self.forward();
            tokens.push(Token::new(TokenType::OperatorAnd, self.range()));
          } else {
            tokens.push(Token::new(TokenType::OperatorRef, self.here()));
          }
        },
        ':' => {
          tokens.push(Token::new(TokenType::OperatorColon, self.here()));
        },
        ',' => {
          tokens.push(Token::new(TokenType::OperatorComma, self.here()));
        },
        '|' => {
          if self.peek() == &'|' {
            self.mark();
            self.forward();
            tokens.push(Token::new(TokenType::OperatorAnd, self.range()));
          } else {
            tokens.push(Token::new(TokenType::OperatorRef, self.here()));
          }
        },
        '0'..='9' => {
          self.mark();
          self.backward();
          let mut token = String::new();
          while !self.is_eof() {
            let curr = self.next();
            match curr {
              '0'..='9' | '.' => {
                token.push(*curr);
              }
              _ => {
                self.backward();
                break
              },
            }
          }
          let num = token.parse().unwrap();
          tokens.push(Token::new(TokenType::LiteralNum(num), self.range()));
        },
        '"' => {
          self.mark();
          let mut token = String::new();
          let mut nf = false;
          while !self.is_eof() {
            let curr = self.next();
            if nf {
              nf = false;
              match curr {
                'n' => {
                  token.push('\n');
                },
                'r' => {
                  token.push('\r');
                },
                't' => {
                  token.push('\t');
                },
                _ => {
                  token.push(*curr);
                },
              }
            } else {
              match curr {
                '"' => {
                  break
                },
                '\\' => {
                  nf = true;
                },
                _ => {
                  token.push(*curr);
                },
              }
            }
          }
          tokens.push(Token::new(TokenType::LiteralStr(token), self.range()));
        },
        '\n' | '\r' => {
          self.line += 1;
          self.mark();
          // self.forward();
          let mut tabs = 0;
          let mut spaces = 0;
          while !self.is_eof() {
            let curr = self.next();
            match curr {
              '\t' => {
                tabs += 1;
              },
              ' ' => {
                spaces += 1;
              }
              _ => {
                self.backward();
                break
              },
            }
          }
          if ignore_nl {
            ignore_nl = false;
          } else {
            tokens.push(Token::new(TokenType::IdentTier(tabs + spaces / (SPACE_SIZE as usize)), self.range()));
          }
        },
        '\\' => {
          ignore_nl = true;
        },
        'a'..='z' | 'A'..='Z' | '_' => {
          self.mark();
          self.backward();
          let mut token = String::new();
          while !self.is_eof() {
            let curr = self.next();
            match curr {
              'a'..='z' | 'A'..='Z' | '_' | '0'..='9' | '\'' => {
                token.push(*curr);
              }
              _ => {
                self.backward();
                break
              },
            }
          }
          match token.as_str() {
            "let" => {
              tokens.push(Token::new(TokenType::KeywordLet, self.range()));
            },
            "struct" => {
              tokens.push(Token::new(TokenType::KeywordStruct, self.range()));
            },
            "enum" => {
              tokens.push(Token::new(TokenType::KeywordEnum, self.range()));
            },
            "trait" => {
              tokens.push(Token::new(TokenType::KeywordTrait, self.range()));
            },
            "fn" => {
              tokens.push(Token::new(TokenType::KeywordFn, self.range()));
            },
            "const" => {
              tokens.push(Token::new(TokenType::KeywordConst, self.range()));
            },
            "impl" => {
              tokens.push(Token::new(TokenType::KeywordImpl, self.range()));
            },
            "true" => {
              tokens.push(Token::new(TokenType::LiteralBool(true), self.range()));
            },
            "false" => {
              tokens.push(Token::new(TokenType::LiteralBool(false), self.range()));
            },
            _ => {
              tokens.push(Token::new(TokenType::Identifier(Rc::new(token)), self.range()));
            },
          }
        },
        _ => {},
      }
    }
    TokenStream::new(tokens)
  }
}