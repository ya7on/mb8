use chumsky::Parser;
use logos::Logos;
use mb8c::{
    ast::{ASTBinaryOp, ASTExpr, ASTFunction, ASTProgram, ASTStmt, ASTType},
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
    assert_eq!(program.unwrap(), ASTProgram { functions: vec![] });
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
        ASTProgram {
            functions: vec![ASTFunction {
                name: "func".to_string(),
                return_type: ASTType::Int,
                params: vec![],
                body: ASTStmt::Block(vec![
                    ASTStmt::Return(None),
                    ASTStmt::Return(Some(ASTExpr::IntLiteral(0)))
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
        ASTProgram {
            functions: vec![ASTFunction {
                name: "func".to_string(),
                return_type: ASTType::Int,
                params: vec![],
                body: ASTStmt::Block(vec![
                    ASTStmt::Declaration {
                        name: "a".to_string(),
                        ty: ASTType::Int,
                        init: None,
                    },
                    ASTStmt::Declaration {
                        name: "a".to_string(),
                        ty: ASTType::Int,
                        init: Some(ASTExpr::IntLiteral(1)),
                    },
                    ASTStmt::Declaration {
                        name: "b".to_string(),
                        ty: ASTType::Char,
                        init: None,
                    },
                    ASTStmt::Declaration {
                        name: "b".to_string(),
                        ty: ASTType::Char,
                        init: Some(ASTExpr::IntLiteral(2)),
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
        ASTProgram {
            functions: vec![ASTFunction {
                name: "func".to_string(),
                return_type: ASTType::Int,
                params: vec![],
                body: ASTStmt::Block(vec![
                    ASTStmt::Expression(ASTExpr::Call {
                        name: "func".to_string(),
                        args: vec![]
                    }),
                    ASTStmt::Expression(ASTExpr::Call {
                        name: "func".to_string(),
                        args: vec![ASTExpr::Var("a".to_string()), ASTExpr::Var("b".to_string())]
                    }),
                    ASTStmt::Expression(ASTExpr::Call {
                        name: "func".to_string(),
                        args: vec![ASTExpr::BinaryOp {
                            op: ASTBinaryOp::Add,
                            lhs: Box::new(ASTExpr::IntLiteral(2)),
                            rhs: Box::new(ASTExpr::IntLiteral(2))
                        }],
                    }),
                    ASTStmt::Expression(ASTExpr::Call {
                        name: "func".to_string(),
                        args: vec![ASTExpr::BinaryOp {
                            op: ASTBinaryOp::Mul,
                            lhs: Box::new(ASTExpr::IntLiteral(2)),
                            rhs: Box::new(ASTExpr::Var("c".to_string()))
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
            ASTProgram {
                functions: vec![ASTFunction {
                    name: "main".to_string(),
                    return_type: ASTType::Int,
                    params: vec![],
                    body: ASTStmt::Block(vec![ASTStmt::If {
                        condition: ASTExpr::IntLiteral(1),
                        then_branch: Box::new(ASTStmt::Block(vec![ASTStmt::Return(Some(
                            ASTExpr::IntLiteral(1)
                        ))])),
                        else_branch: Some(Box::new(ASTStmt::Block(vec![ASTStmt::Return(Some(
                            ASTExpr::IntLiteral(2)
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
            ASTProgram {
                functions: vec![ASTFunction {
                    name: "main".to_string(),
                    return_type: ASTType::Int,
                    params: vec![],
                    body: ASTStmt::Block(vec![ASTStmt::While {
                        condition: ASTExpr::IntLiteral(1),
                        body: Box::new(ASTStmt::Block(vec![ASTStmt::Return(Some(
                            ASTExpr::IntLiteral(1)
                        ))])),
                    }])
                }]
            }
        );
    }
}
