#[derive(Debug)]
pub struct HIRProgram {
    pub functions: Vec<HIRFunction>,
}

#[derive(Debug)]
pub struct HIRFunction {
    pub id: FunctionId,
    pub params: Vec<SymbolId>,
    pub body: Vec<HIRStmt>,
}

#[derive(Debug)]
pub struct SymbolId(pub usize);

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct TypeId(pub usize);

#[derive(Debug)]
pub struct FunctionId();

#[derive(Debug)]
pub enum Literal {
    Int(i32),
}

#[derive(Debug)]
pub enum HIRStmt {
    Block(Vec<HIRStmt>),
    Declaration {
        symbol: SymbolId,
        ty: TypeId,
        init: Option<HIRExpr>,
    },
    If {
        cond: Box<HIRExpr>,
        then: Box<HIRStmt>,
        else_: Option<Box<HIRStmt>>,
    },
    While {
        cond: Box<HIRExpr>,
        body: Box<HIRStmt>,
    },
    Return(Option<HIRExpr>),
    Expr(HIRExpr),
}

#[derive(Debug)]
pub enum HIRBinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum HIRUnaryOp {
    Neg,
}

#[derive(Debug)]
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
        func: FunctionId,
        args: Vec<HIRExpr>,
        ty: TypeId,
    },
}
