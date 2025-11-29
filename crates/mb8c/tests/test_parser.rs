use mb8c::{
    parser::{
        ast::{Expr, Function, Program, Stmt, Type},
        base::Parser,
    },
    tokenizer::lexer::Lexer,
};

#[test]
fn test_main() {
    let src = r#"
        int main(int a, int b) {
            return 0;
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
                body: Stmt::Block(vec![Stmt::Return(Some(Expr::IntLiteral(0)))])
            }]
        }
    );
}
