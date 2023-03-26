use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Type {
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
    Ref(Box<Type>),
    Array(Box<Type>, usize),
    Unit(Vec<Type>),
    // Impled(&'a TraitDef<'a>),
    // Struct(&'a StructDef<'a>),
    // Enum(&'a EnumDef<'a>),
    // _NeverUseful(&'a str), // this is used for saving the lifetime 'a
    Costume(Rc<String>),
    Anna(Box<Type>, Vec<Type>),
}

#[derive(Debug, Clone)]
pub struct FnArg {
    pub name: Rc<String>,
}

impl FnArg {
    pub fn new(name: Rc<String>) -> Self {
        Self { name }
    }
}

#[derive(Debug, Clone)]
pub struct FnDef {
    pub name: Rc<String>,
    pub arguments: Vec<FnArg>,
}

#[derive(Debug)]
pub struct StructField {
    pub name: Rc<String>,
}

#[derive(Debug)]
pub struct StructDef {
    pub name: Rc<String>,
    pub fields: Vec<StructField>,
}

#[derive(Debug)]
pub struct EnumField {
    pub name: Rc<String>,
    pub values: Vec<Rc<String>>,
}

#[derive(Debug)]
pub struct EnumDef {
    pub name: Rc<String>,
    pub fields: Vec<EnumField>,
}

#[derive(Debug)]
pub struct TraitField {
    pub body: FnDef,
}

#[derive(Debug)]
pub struct TraitDef {
    pub name: Rc<String>,
    pub fields: Vec<TraitField>,
}

#[derive(Debug)]
pub struct VarDef {
    pub name: Rc<String>,
}

#[derive(Debug, Clone)]
pub enum Scope {
    Global,
    Block(usize, usize), // from token# to token#
}

pub enum ScopeOrdering {
    Contains,
    Cross,
    RContains,
}

impl Scope {
    pub fn order(&self, other: &Self) -> ScopeOrdering {
        match self {
            Scope::Global => ScopeOrdering::Contains,
            Scope::Block(lf, lt) => match other {
                Scope::Global => ScopeOrdering::RContains,
                Scope::Block(rf, rt) => {
                    if rf > lf && rt < lt {
                        // lf  rf  rt  lt
                        ScopeOrdering::Contains
                    } else if rf < lf && rt > lt {
                        // rf  lf  lt  rt
                        ScopeOrdering::RContains
                    } else {
                        ScopeOrdering::Cross
                    }
                }
            },
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
            body,
            scope: Scope::Global,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AstNode {
    Program(AstProgram),
    Fn(FnDef, AstBlock),
    Impl(Rc<String>, Vec<AstNode>),
    Let(Rc<String>, Option<AstExpr>),
    Expr(AstExpr),
    // Block(AstBlock),
    Empty,
}

#[derive(Debug, Clone)]
pub struct AstBlock {
    pub block: Vec<AstNode>,
}

impl AstBlock {
    pub fn new() -> Self {
        Self { block: Vec::new() }
    }

    pub fn add(&mut self, ast_node: AstNode) {
        self.block.push(ast_node);
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum AstExpr {
    Neg(Box<AstExpr>),
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
    Not(Box<AstExpr>),
    And(Box<AstExpr>, Box<AstExpr>),
    Or(Box<AstExpr>, Box<AstExpr>),
    Assign(Rc<String>, Box<AstExpr>),
    FnCall(Box<AstExpr>, Vec<AstExpr>),
    Var(Rc<String>),

    Block(AstBlock),

    IfElse(Box<AstExpr>, AstBlock, AstBlock),
    While(Box<AstExpr>, AstBlock),
    // For(Rc<String>, Box<AstExpr>, AstBlock)
    LiteralNum(f64),
    LiteralStr(String),
    LiteralUnit(Vec<AstExpr>),
    LiteralArray(Vec<AstExpr>),
}
