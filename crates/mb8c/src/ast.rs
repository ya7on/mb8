#[derive(Clone, Debug, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ASTType {
    Void,
    Unsigned8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ASTProgram {
    pub functions: Vec<ASTFunction>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ASTFunction {
    pub name: String,
    pub return_type: ASTType,
    pub params: Vec<(String, ASTType)>,
    pub vars: Vec<(String, ASTType)>,
    pub body: ASTStmt,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ASTStmt {
    Block(Vec<ASTStmt>),
    Return {
        expr: Option<ASTExpr>,
        span: Span,
    },
    Expression {
        expr: ASTExpr,
        span: Span,
    },
    If {
        condition: ASTExpr,
        then_branch: Box<ASTStmt>,
        else_branch: Option<Box<ASTStmt>>,
        span: Span,
    },
    While {
        condition: ASTExpr,
        body: Box<ASTStmt>,
        span: Span,
    },
    Assign {
        name: String,
        value: ASTExpr,
        span: Span,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum ASTBinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ASTUnaryOp {
    Neg,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ASTExpr {
    IntLiteral {
        value: u16,
        span: Span,
    },
    BinaryOp {
        op: ASTBinaryOp,
        lhs: Box<ASTExpr>,
        rhs: Box<ASTExpr>,
        span: Span,
    },
    UnaryOp {
        op: ASTUnaryOp,
        expr: Box<ASTExpr>,
        span: Span,
    },
    Var {
        name: String,
        span: Span,
    },
    Call {
        name: String,
        args: Vec<ASTExpr>,
        span: Span,
    },
}
