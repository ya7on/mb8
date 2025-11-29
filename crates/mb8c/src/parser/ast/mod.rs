pub mod expr;
pub mod function;
pub mod helpers;
pub mod program;
pub mod stmt;

#[derive(Debug, PartialEq)]
pub enum Type {
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
    Return(Option<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    IntLiteral(i16),
}
