use std::rc::Rc;

use crate::{
    build_ti_error,
    frontend::lexer::token::{TokenStream, TokenType},
};

use super::ast::*;

#[derive(Debug)]
pub struct Parser {
    sym_id: usize,
    pub fn_def: Vec<WithScope<FnDef>>,
    pub struct_def: Vec<WithScope<StructDef>>,
    pub enum_def: Vec<WithScope<EnumDef>>,
    pub trait_def: Vec<WithScope<TraitDef>>,
    pub var_def: Vec<WithScope<VarDef>>,
    pub tokens: TokenStream,
    pub ast: AstNode,
}

impl Parser {
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

impl Parser {
    pub fn parse(&mut self) {
        let mut program = AstProgram::new();
        while !self.tokens.is_eof() {
            // if !self.tokens.assert_next_tier(0) { build_ti_error!(@at self.tokens.peek(), @err "Unexpected Token `{:?}`", self.tokens.peek()) }
            // if self.tokens.is_eof() { break }
            if let Some(ast_node) = self.parse_definion(Scope::Global) {
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

    /* fn sym_name(&mut self) -> Rc<String> {
        self.sym_id += 1;
        Rc::new(format!("${}", self.sym_id))
    } */

    fn parse_definion(&mut self, scope: Scope) -> Option<Option<AstNode>> {
        let token = self.tokens.next();
        match token.t_type {
            TokenType::KeywordFn => Some(Some(self.parse_fn_definion(scope))),
            TokenType::KeywordEnum => {
                self.parse_enum_definion(scope);
                Some(None)
            }
            TokenType::KeywordStruct => {
                self.parse_struct_definion(scope);
                Some(None)
            }
            TokenType::KeywordTrait => {
                self.parse_trait_definion(scope);
                Some(None)
            }
            TokenType::KeywordImpl => Some(self.parse_impl_definion(scope)),
            TokenType::KeywordLet => {
                if let TokenType::Identifier(x) = &self.tokens.next().t_type {
                    let name = x.clone();
                    let expr = if self.tokens.assert_next(TokenType::OperatorAssign) {
                        let expr = self.parse_expr();
                        Some(expr)
                    } else {
                        None
                    };
                    Some(Some(AstNode::Let(name, expr)))
                } else {
                    build_ti_error!(@at self.tokens.last(), @err "Expect Identifier, found {:?}", self.tokens.last())
                }
            }
            _ => {
                self.tokens.backward();
                self.tokens.backward();
                None
            }
        }
    }

    fn parse_fn_definion(&mut self, _scope: Scope) -> AstNode {
        let fname;
        let mut fargs: Vec<FnArg> = Vec::new();
        match self.tokens.next().t_type.clone() {
            TokenType::OperatorLes => {
                // fn<TN: TT[, ...]> FN(FA: FT[, ...])
                todo!()
            }
            TokenType::Identifier(n) => {
                // fn FN(FA: FT[, ...])
                fname = n;
            }
            _ => {
                build_ti_error!(@at self.tokens.peek(), @err "Unexpect Token `{:?}`", self.tokens.peek())
            }
        }
        if !self.tokens.assert_next(TokenType::OpenParen) {
            build_ti_error!(@at self.tokens.peek(), @err "Expect Token `(`, found `{:?}`", self.tokens.peek())
        }
        loop {
            if self.tokens.assert_next(TokenType::CloseParen) {
                break;
            }
            if let TokenType::Identifier(argn) = self.tokens.next().t_type.clone() {
                let arg = FnArg::new(argn.clone());
                fargs.push(arg);
            } else {
                build_ti_error!(@at self.tokens.last(), @err "Expect Identifier or Token `)`, found {:?}", self.tokens.last())
            }
            if self.tokens.assert_next(TokenType::CloseParen) {
                break;
            }
            self.tokens.forward();
            // self.tokens.forward();
            if self.tokens.is_eof() {
                build_ti_error!(@err "Expect Token `)`, found `Eof`")
            }
        }
        self.fn_def.push(WithScope::global(FnDef {
            name: fname.clone(),
            arguments: fargs.clone(),
        }));

        let mut fbody;
        if self.tokens.assert_next(TokenType::OperatorFatArrow) {
            let expr = self.parse_expr();
            fbody = AstBlock::new();
            fbody.add(AstNode::Expr(expr));
        } else {
            fbody = self.parse_block();
        }

        AstNode::Fn(
            FnDef {
                name: fname.clone(),
                arguments: fargs,
            },
            fbody,
        )
    }

    fn parse_enum_definion(&mut self, scope: Scope) {
        todo!()
    }

    fn parse_struct_definion(&mut self, scope: Scope) {
        todo!()
    }

    fn parse_trait_definion(&mut self, scope: Scope) {
        todo!()
    }

    fn parse_impl_definion(&mut self, scope: Scope) -> Option<AstNode> {
        todo!()
    }

    fn parse_expr(&mut self) -> AstExpr {
        self.parse_expr_logic()
    }

    fn parse_expr_primary(&mut self) -> AstExpr {
        let curr = self.tokens.next();
        match &curr.t_type {
            TokenType::Identifier(x) => {
                let name = x.clone();
                if self.tokens.assert_next(TokenType::OperatorAssign) {
                    let expr = self.parse_expr();
                    AstExpr::Assign(name, Box::new(expr))
                } else {
                    AstExpr::Var(name)
                }
            }
            TokenType::LiteralNum(x) => AstExpr::LiteralNum(*x),
            TokenType::LiteralStr(x) => AstExpr::LiteralStr(x.clone()),
            TokenType::OperatorSub => {
                let expr = self.parse_expr();
                AstExpr::Neg(Box::new(expr))
            }
            TokenType::KeywordIf => {
                let expr = self.parse_expr();
                let true_case = self.parse_block();
                let false_case = if self.tokens.assert_next(TokenType::KeywordElse) {
                    self.parse_block()
                } else {
                    AstBlock::new()
                };
                AstExpr::IfElse(Box::new(expr), true_case, false_case)
            }
            TokenType::KeywordWhile => {
                let expr = self.parse_expr();
                let body = self.parse_block();
                /* let else = if self.tokens.assert_next(TokenType::KeywordElse) {
                    self.parse_block()
                } else {
                    AstBlock::new()
                }; */
                AstExpr::While(Box::new(expr), body)
            }
            TokenType::OpenBracket => {
                self.tokens.backward();
                AstExpr::Block(self.parse_block())
            }
            TokenType::OpenParen => {
                let expr = self.parse_expr();
                if !self.tokens.assert_next(TokenType::CloseParen) {
                    build_ti_error!(@at self.tokens.peek(), @err "Expect Token `)`, found {:?}.", self.tokens.peek())
                }
                expr
            }
            _ => {
                build_ti_error!(@at curr, @err "Unexpected Token `{:?}` While Parsing an Expr.", curr)
            }
        }
    }

    fn parse_expr_logic(&mut self) -> AstExpr {
        let mut lhs = self.parse_expr_comp();
        while !self.tokens.is_eof() {
            let curr = self.tokens.peek();
            match curr.t_type {
                TokenType::OperatorAnd => {
                    self.tokens.forward();
                    let rhs = self.parse_expr_comp();
                    lhs = AstExpr::And(Box::new(lhs), Box::new(rhs));
                }
                TokenType::OperatorOr => {
                    self.tokens.forward();
                    let rhs = self.parse_expr_comp();
                    lhs = AstExpr::Or(Box::new(lhs), Box::new(rhs));
                }
                _ => break,
            }
        }
        lhs
    }
    fn parse_expr_comp(&mut self) -> AstExpr {
        let mut lhs = self.parse_expr_add();
        while !self.tokens.is_eof() {
            let curr = self.tokens.peek();
            match curr.t_type {
                TokenType::OperatorLes => {
                    self.tokens.forward();
                    let rhs = self.parse_expr_add();
                    lhs = AstExpr::Les(Box::new(lhs), Box::new(rhs));
                }
                TokenType::OperatorGrt => {
                    self.tokens.forward();
                    let rhs = self.parse_expr_add();
                    lhs = AstExpr::Grt(Box::new(lhs), Box::new(rhs));
                }
                TokenType::OperatorLeq => {
                    self.tokens.forward();
                    let rhs = self.parse_expr_add();
                    lhs = AstExpr::Leq(Box::new(lhs), Box::new(rhs));
                }
                TokenType::OperatorGeq => {
                    self.tokens.forward();
                    let rhs = self.parse_expr_add();
                    lhs = AstExpr::Geq(Box::new(lhs), Box::new(rhs));
                }
                _ => break,
            }
        }
        lhs
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
                }
                TokenType::OperatorSub => {
                    self.tokens.forward();
                    let rhs = self.parse_expr_mul();
                    lhs = AstExpr::Sub(Box::new(lhs), Box::new(rhs));
                }
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
                }
                TokenType::OperatorDiv => {
                    self.tokens.forward();
                    let rhs = self.parse_expr_call();
                    lhs = AstExpr::Div(Box::new(lhs), Box::new(rhs));
                }
                _ => break,
            }
        }
        lhs
    }

    fn parse_expr_call(&mut self) -> AstExpr {
        let member = self.parse_expr_primary();
        if self.tokens.assert_next(TokenType::OpenParen) {
            let mut args = Vec::new();
            if !self.tokens.assert_next(TokenType::CloseParen) {
                loop {
                    let expr = self.parse_expr();
                    args.push(expr);
                    if self.tokens.assert_next(TokenType::CloseParen) {
                        break;
                    }
                    self.tokens.forward();
                }
            }
            AstExpr::FnCall(Box::new(member), args)
        } else {
            member
        }
    }

    fn find_scope(&self) -> Scope {
        let mut idx = self.tokens.curr;
        while !self.tokens.is_eof() {
            match self.tokens.at(idx).unwrap().t_type {
                TokenType::CloseBracket => break,
                _ => {}
            }
            idx += 1;
        }
        Scope::Block(self.tokens.curr, idx)
    }

    fn parse_block(&mut self) -> AstBlock {
        if !self.tokens.assert_next(TokenType::OpenBracket) {
            build_ti_error!(@at self.tokens.peek(), @err "Expect Token `{{`, found {:?}.", self.tokens.peek())
        }
        let mut block = AstBlock::new();
        let scope = self.find_scope();
        while !self.tokens.is_eof() && !self.tokens.assert_next(TokenType::CloseBracket) {
            let ast_node = self.parse_stmt(&scope);
            block.add(ast_node);
        }
        block
    }

    fn parse_stmt(&mut self, scope: &Scope) -> AstNode {
        if let Some(ast_node) = self.parse_definion(scope.clone()) {
            ast_node.unwrap_or(AstNode::Empty)
        } else {
            self.tokens.forward();
            let expr = self.parse_expr();
            AstNode::Expr(expr)
        }
    }

    fn parse_type(&mut self) -> Type {
        self.parse_type_anna()
    }

    fn parse_type_anna(&mut self) -> Type {
        let t = self.parse_type_primary();
        if self.tokens.assert_next(TokenType::OperatorLes) {
            let mut anna = Vec::new();
            loop {
                let expr = self.parse_type();
                anna.push(expr);
                if self.tokens.assert_next(TokenType::OperatorGrt) {
                    break;
                }
                self.tokens.forward();
            }
            Type::Anna(Box::new(t), anna)
        } else {
            t
        }
    }

    fn parse_type_primary(&mut self) -> Type {
        let curr = self.tokens.next();
        match &curr.t_type {
            TokenType::Identifier(t) => match t.as_str() {
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
                _ => Type::Costume(t.clone()),
            },
            TokenType::OpenParen => {
                let mut unit = Vec::new();
                loop {
                    let expr = self.parse_type();
                    unit.push(expr);
                    if self.tokens.assert_next(TokenType::CloseParen) {
                        break;
                    }
                    self.tokens.forward();
                }
                Type::Unit(unit)
            }
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
            }
            TokenType::OperatorRef => {
                let t = self.parse_type();
                Type::Ref(Box::new(t))
            }
            _ => {
                build_ti_error!(@at curr, @err "Expect Identifier, found {:?}", curr)
            }
        }
    }
}
