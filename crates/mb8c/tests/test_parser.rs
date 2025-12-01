use mb8c::{
    parser::{
        ast::{Expr, Function, Program, Stmt, Type},
        base::Parser,
    },
    tokenizer::{lexer::Lexer, token::Operator},
};

#[test]
fn test_empty_program() {
    let src = r#""#;
    let tokens = Lexer::new(src).tokenize().unwrap();
    let mut program = Parser::new(tokens);
    assert_eq!(
        program.parse_program().unwrap(),
        Program { functions: vec![] }
    );
}

#[test]
fn test_return() {
    let src = r#"
        int func() {
            return;
            return 0;
        }
    "#;
    let tokens = Lexer::new(src).tokenize().unwrap();
    let mut program = Parser::new(tokens);
    assert_eq!(
        program.parse_program().unwrap(),
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
    let src = r#"
        int func() {
            int a;
            int a = 1;
            char b;
            char b = 2;
        }
    "#;
    let tokens = Lexer::new(src).tokenize().unwrap();
    let mut program = Parser::new(tokens);
    assert_eq!(
        program.parse_program().unwrap(),
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
    let src = r#"
        int func() {
            func();
            func(a, b);
            func(2 + 2);
            func(2 * c);
        }
    "#;
    let tokens = Lexer::new(src).tokenize().unwrap();
    let mut program = Parser::new(tokens);
    assert_eq!(
        program.parse_program().unwrap(),
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
                            op: Operator::Plus,
                            lhs: Box::new(Expr::IntLiteral(2)),
                            rhs: Box::new(Expr::IntLiteral(2))
                        }],
                    }),
                    Stmt::Expression(Expr::Call {
                        name: "func".to_string(),
                        args: vec![Expr::BinaryOp {
                            op: Operator::Asterisk,
                            lhs: Box::new(Expr::IntLiteral(2)),
                            rhs: Box::new(Expr::Var("c".to_string()))
                        }],
                    })
                ])
            }]
        }
    );
}
