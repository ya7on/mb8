#[derive(Debug)]
pub struct HIRProgram {
    pub functions: Vec<HIRFunction>,
}

#[derive(Debug)]
pub struct HIRFunction {
    pub id: SymbolId,
    pub params: Vec<SymbolId>,
    pub body: Vec<HIRStmt>,
}

#[derive(Debug, Copy, Clone)]
pub struct SymbolId(pub usize);

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct TypeId(pub usize);

#[derive(Debug)]
pub enum Literal {
    Int(i16),
}

#[derive(Debug)]
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
}

#[derive(Debug)]
pub enum HIRBinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
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
        func: SymbolId,
        args: Vec<HIRExpr>,
        ty: TypeId,
    },
    Assign {
        symbol: SymbolId,
        value: Box<HIRExpr>,
        ty: TypeId,
    },
}
