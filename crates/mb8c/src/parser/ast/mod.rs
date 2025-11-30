use crate::tokenizer::token::Operator;

pub mod expr;
pub mod function;
pub mod helpers;
pub mod program;
pub mod stmt;

#[derive(Debug, PartialEq)]
pub enum Type {
    Void,
    Int,
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub functions: Vec<Function>,
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub return_type: Type,
    pub args: Vec<(String, Type)>,
    pub body: Stmt,
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Declaration {
        name: String,
        ty: Type,
        init: Option<Expr>,
    },
    Return(Option<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    IntLiteral(i16),
    BinaryOp {
        op: Operator,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Negation(Box<Expr>),
}
