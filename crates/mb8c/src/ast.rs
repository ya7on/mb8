#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ASTType {
    Void,
    Char,
    Int,
}

impl ASTType {
    #[must_use]
    pub fn size_in_bytes(&self) -> u8 {
        match self {
            ASTType::Void => 0,
            ASTType::Char => 1,
            ASTType::Int => 2,
        }
    }
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
    pub body: ASTStmt,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ASTStmt {
    Block(Vec<ASTStmt>),
    Declaration {
        name: String,
        ty: ASTType,
        init: Option<ASTExpr>,
    },
    Return(Option<ASTExpr>),
    Expression(ASTExpr),
    If {
        condition: ASTExpr,
        then_branch: Box<ASTStmt>,
        else_branch: Option<Box<ASTStmt>>,
    },
    While {
        condition: ASTExpr,
        body: Box<ASTStmt>,
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
    IntLiteral(i16),
    BinaryOp {
        op: ASTBinaryOp,
        lhs: Box<ASTExpr>,
        rhs: Box<ASTExpr>,
    },
    UnaryOp {
        op: ASTUnaryOp,
        expr: Box<ASTExpr>,
    },
    Var(String),
    Assign {
        name: String,
        value: Box<ASTExpr>,
    },
    Call {
        name: String,
        args: Vec<ASTExpr>,
    },
}
