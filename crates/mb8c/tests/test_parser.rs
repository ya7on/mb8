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
                args: vec![("a".to_string(), Type::Int), ("b".to_string(), Type::Int)],
                body: Stmt::Block(vec![
                    Stmt::Declaration {
                        name: "a".to_string(),
                        ty: Type::Int,
                        init: Some(Expr::IntLiteral(1))
                    },
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
