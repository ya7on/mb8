use chumsky::Parser;
use logos::Logos;
use mb8c::{
    ast::{BinaryOp, Expr, Function, Program, Stmt, Type},
    parser::program::program_parser,
    tokens::TokenKind,
};

#[test]
fn test_empty_program() {
    let src = r"";
    let tokens = TokenKind::lexer(src)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let parser = program_parser();
    let program = parser.parse(&tokens);
    assert_eq!(program.unwrap(), Program { functions: vec![] });
}

#[test]
fn test_return() {
    let src = r"
        int func() {
            return;
            return 0;
        }
    ";
    let tokens = TokenKind::lexer(src)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let parser = program_parser();
    let program = parser.parse(&tokens);
    assert_eq!(
        program.unwrap(),
        Program {
            functions: vec![Function {
                name: "func".to_string(),
                return_type: Type::Int,
                params: vec![],
                body: Stmt::Block(vec![
                    Stmt::Return(None),
                    Stmt::Return(Some(Expr::IntLiteral(0)))
                ])
            }]
        }
    );
}

#[test]
fn test_variables() {
    let src = r"
        int func() {
            int a;
            int a = 1;
            char b;
            char b = 2;
        }
    ";
    let tokens = TokenKind::lexer(src)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let parser = program_parser();
    let program = parser.parse(&tokens);
    assert_eq!(
        program.unwrap(),
        Program {
            functions: vec![Function {
                name: "func".to_string(),
                return_type: Type::Int,
                params: vec![],
                body: Stmt::Block(vec![
                    Stmt::Declaration {
                        name: "a".to_string(),
                        ty: Type::Int,
                        init: None,
                    },
                    Stmt::Declaration {
                        name: "a".to_string(),
                        ty: Type::Int,
                        init: Some(Expr::IntLiteral(1)),
                    },
                    Stmt::Declaration {
                        name: "b".to_string(),
                        ty: Type::Char,
                        init: None,
                    },
                    Stmt::Declaration {
                        name: "b".to_string(),
                        ty: Type::Char,
                        init: Some(Expr::IntLiteral(2)),
                    },
                ])
            }]
        }
    );
}

#[test]
fn test_call() {
    let src = r"
        int func() {
            func();
            func(a, b);
            func(2 + 2);
            func(2 * c);
        }
    ";
    let tokens = TokenKind::lexer(src)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let parser = program_parser();
    let program = parser.parse(&tokens);
    assert_eq!(
        program.unwrap(),
        Program {
            functions: vec![Function {
                name: "func".to_string(),
                return_type: Type::Int,
                params: vec![],
                body: Stmt::Block(vec![
                    Stmt::Expression(Expr::Call {
                        name: "func".to_string(),
                        args: vec![]
                    }),
                    Stmt::Expression(Expr::Call {
                        name: "func".to_string(),
                        args: vec![Expr::Var("a".to_string()), Expr::Var("b".to_string())]
                    }),
                    Stmt::Expression(Expr::Call {
                        name: "func".to_string(),
                        args: vec![Expr::BinaryOp {
                            op: BinaryOp::Add,
                            lhs: Box::new(Expr::IntLiteral(2)),
                            rhs: Box::new(Expr::IntLiteral(2))
                        }],
                    }),
                    Stmt::Expression(Expr::Call {
                        name: "func".to_string(),
                        args: vec![Expr::BinaryOp {
                            op: BinaryOp::Mul,
                            lhs: Box::new(Expr::IntLiteral(2)),
                            rhs: Box::new(Expr::Var("c".to_string()))
                        }],
                    })
                ])
            }]
        }
    );
}

#[test]
fn test_if_statement() {
    {
        let src = r"
        int main() {
            if (1) {
                return 1;
            } else {
                return 2;
            }
        }
        ";
        let tokens = TokenKind::lexer(src)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let parser = program_parser();
        let program = parser.parse(&tokens);
        assert_eq!(
            program.unwrap(),
            Program {
                functions: vec![Function {
                    name: "main".to_string(),
                    return_type: Type::Int,
                    params: vec![],
                    body: Stmt::Block(vec![Stmt::If {
                        condition: Expr::IntLiteral(1),
                        then_branch: Box::new(Stmt::Block(vec![Stmt::Return(Some(
                            Expr::IntLiteral(1)
                        ))])),
                        else_branch: Some(Box::new(Stmt::Block(vec![Stmt::Return(Some(
                            Expr::IntLiteral(2)
                        ))])))
                    },])
                }]
            }
        );
    }
}

#[test]
fn test_while_statement() {
    {
        let src = r"
        int main() {
            while (1) {
                return 1;
            }
        }
        ";
        let tokens = TokenKind::lexer(src)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let parser = program_parser();
        let program = parser.parse(&tokens);
        assert_eq!(
            program.unwrap(),
            Program {
                functions: vec![Function {
                    name: "main".to_string(),
                    return_type: Type::Int,
                    params: vec![],
                    body: Stmt::Block(vec![Stmt::While {
                        condition: Expr::IntLiteral(1),
                        body: Box::new(Stmt::Block(vec![Stmt::Return(Some(Expr::IntLiteral(1)))])),
                    }])
                }]
            }
        );
    }
}
