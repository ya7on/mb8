use crate::hir::{symbols::SymbolTable, types::TypeTable};

#[derive(Debug, Clone)]
pub struct HIRGlobal {
    pub symbol: SymbolId,
    pub type_id: TypeId,
    pub at: usize,
}

#[derive(Debug, Clone)]
pub struct HIRProgram {
    pub functions: Vec<HIRFunction>,
    pub symbols: SymbolTable,
    pub types: TypeTable,
    pub globals: Vec<HIRGlobal>,
}

#[derive(Debug, Clone)]
pub struct HIRFunctionParam {
    pub symbol: SymbolId,
    pub type_id: TypeId,
    pub index: usize,
}

#[derive(Debug, Clone)]
pub struct HIRFunctionLocal {
    pub symbol: SymbolId,
    pub type_id: TypeId,
}

#[derive(Debug, Clone)]
pub struct HIRFunction {
    pub id: SymbolId,
    pub name: String,
    pub params: Vec<HIRFunctionParam>,
    pub locals: Vec<HIRFunctionLocal>,
    pub body: Vec<HIRStmt>,
    pub params_size: usize,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct SymbolId(pub usize);

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct TypeId(pub usize);

#[derive(Debug, Clone, Copy)]
pub enum Literal {
    Int(u16),
}

#[derive(Debug, Clone)]
pub enum HIRStmt {
    Block(Vec<HIRStmt>),
    Return(Option<HIRExpr>),
    Expression(HIRExpr),
    If {
        condition: Box<HIRExpr>,
        then_branch: Box<HIRStmt>,
        else_branch: Option<Box<HIRStmt>>,
    },
    While {
        condition: Box<HIRExpr>,
        body: Box<HIRStmt>,
    },
    Assign {
        symbol: SymbolId,
        ty: TypeId,
        value: HIRExpr,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum HIRBinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
}

#[derive(Debug, Clone, Copy)]
pub enum HIRUnaryOp {
    Neg,
}

#[derive(Debug, Clone)]
pub enum HIRExpr {
    Var {
        symbol: SymbolId,
        ty: TypeId,
    },
    Literal {
        literal: Literal,
        ty: TypeId,
    },
    Binary {
        op: HIRBinaryOp,
        lhs: Box<HIRExpr>,
        rhs: Box<HIRExpr>,
        ty: TypeId,
    },
    Unary {
        op: HIRUnaryOp,
        expr: Box<HIRExpr>,
        ty: TypeId,
    },
    Call {
        symbol: SymbolId,
        label: String,
        args: Vec<HIRExpr>,
        ty: TypeId,
    },
}
