#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Type {
    Void,
    Char,
    Int,
}

impl Type {
    #[must_use]
    pub fn size_in_bytes(&self) -> u8 {
        match self {
            Type::Void => 0,
            Type::Char => 1,
            Type::Int => 2,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub functions: Vec<Function>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub return_type: Type,
    pub params: Vec<(String, Type)>,
    pub body: Stmt,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Declaration {
        name: String,
        ty: Type,
        init: Option<Expr>,
    },
    Return(Option<Expr>),
    Expression(Expr),
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    IntLiteral(i16),
    BinaryOp {
        op: BinaryOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Negation(Box<Expr>),
    Var(String),
    Assign {
        name: String,
        value: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
}
