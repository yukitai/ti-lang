pub mod TiBC {
  pub const LOAD_CONST: u16   = 0;
  pub const LOAD_SESSION: u16 = 1;
  pub const BUILD_FUNC: u16   = 2;
  pub const BIN_PRINT: u16    = 3;
  pub const ADD: u16          = 4;
  pub const SUB: u16          = 5;
  pub const MUL: u16          = 6;
  pub const DIV: u16          = 7;
  pub const MOD: u16          = 8;
  pub const AND: u16          = 9;
  pub const OR: u16           = 10;
  pub const NOT: u16          = 11;
  pub const LES: u16          = 12;
  pub const GRT: u16          = 13;
  pub const LEQ: u16          = 14;
  pub const GEQ: u16          = 15;
  pub const EQ: u16           = 16;
  pub const NEQ: u16          = 17;
  pub const JMP: u16          = 18;
  pub const JNZ: u16          = 19;
  pub const JZ: u16           = 20;
  pub const EXIT: u16           = 65535;
}

pub struct TiByteCode<'a> {
  bytecode: &'a [u16],
  curr: usize,
}

impl<'a> TiByteCode<'a> {
  pub fn new(bytecode: &'a [u16]) -> Self {
    Self { bytecode, curr: 0,  }
  }
  pub fn has(&self) -> bool {
    self.curr < self.bytecode.len()
  }
  pub fn next(&mut self, l: usize) {
    self.curr += l;
  }
  pub fn at(&self) -> u16 {
    *self.bytecode.get(self.curr).unwrap()
  }
}

impl<'a> std::fmt::Debug for TiByteCode<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self.bytecode)
  }
}

pub struct TiByteCodeBuilder {

}