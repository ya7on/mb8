use mb8c::{
    parser::{
        ast::{Expr, Function, Program, Stmt, Type},
        base::Parser,
    },
    tokenizer::{lexer::Lexer, token::Operator},
};

#[test]
fn test_main() {
    let src = r#"
        int main(int a, int b) {
            int a = 1;
            a = a + 1;
            func();
            func(a, 3, 2 + variable);
            return (1 + -1) * 2;
        }
    "#;
    let tokens = Lexer::new(src).tokenize().unwrap();
    let mut program = Parser::new(tokens);
    assert_eq!(
        program.parse_program().unwrap(),
        Program {
            functions: vec![Function {
                name: "main".to_string(),
                return_type: Type::Int,
                params: vec![("a".to_string(), Type::Int), ("b".to_string(), Type::Int)],
                body: Stmt::Block(vec![
                    Stmt::Declaration {
                        name: "a".to_string(),
                        ty: Type::Int,
                        init: Some(Expr::IntLiteral(1))
                    },
                    Stmt::Expression(Expr::Assign {
                        name: "a".to_string(),
                        value: Box::new(Expr::BinaryOp {
                            op: Operator::Plus,
                            lhs: Box::new(Expr::Var("a".to_string())),
                            rhs: Box::new(Expr::IntLiteral(1))
                        })
                    }),
                    Stmt::Expression(Expr::Call {
                        name: "func".to_owned(),
                        args: vec![]
                    }),
                    Stmt::Expression(Expr::Call {
                        name: "func".to_owned(),
                        args: vec![
                            Expr::Var("a".to_owned()),
                            Expr::IntLiteral(3),
                            Expr::BinaryOp {
                                op: Operator::Plus,
                                lhs: Box::new(Expr::IntLiteral(2)),
                                rhs: Box::new(Expr::Var("variable".to_owned()))
                            }
                        ],
                    }),
                    Stmt::Return(Some(Expr::BinaryOp {
                        op: Operator::Asterisk,
                        lhs: Box::new(Expr::BinaryOp {
                            op: Operator::Plus,
                            lhs: Box::new(Expr::IntLiteral(1)),
                            rhs: Box::new(Expr::Negation(Box::new(Expr::IntLiteral(1)))),
                        }),
                        rhs: Box::new(Expr::IntLiteral(2)),
                    }))
                ])
            }]
        }
    );
}
