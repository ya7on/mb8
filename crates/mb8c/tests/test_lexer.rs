use mb8c::tokenizer::{
    lexer::Lexer,
    token::{Keyword, Operator, TokenKind},
};

#[test]
fn test_main() {
    let src = r#"
        int main(int a, int b) {
            int a = 1;
            int b = 2 + 2;
            int c = a + b;
            return 1 + 2 * 3 / 4;
        }
    "#;
    let result = Lexer::new(src).tokenize();
    assert_eq!(
        result
            .unwrap()
            .into_iter()
            .map(|token| token.kind)
            .collect::<Vec<_>>(),
        vec![
            // Function declaration
            TokenKind::Keyword(Keyword::Int),
            TokenKind::Ident("main".to_string()),
            // Args
            TokenKind::LeftParenthesis,
            TokenKind::Keyword(Keyword::Int),
            TokenKind::Ident("a".to_string()),
            TokenKind::Comma,
            TokenKind::Keyword(Keyword::Int),
            TokenKind::Ident("b".to_string()),
            TokenKind::RightParenthesis,
            // Function body
            TokenKind::LeftBrace,
            // int a = 1;
            TokenKind::Keyword(Keyword::Int),
            TokenKind::Ident("a".to_string()),
            TokenKind::Operator(Operator::Eq),
            TokenKind::Number(1),
            TokenKind::Semicolon,
            // int b = 2;
            TokenKind::Keyword(Keyword::Int),
            TokenKind::Ident("b".to_string()),
            TokenKind::Operator(Operator::Eq),
            TokenKind::Number(2),
            TokenKind::Operator(Operator::Plus),
            TokenKind::Number(2),
            TokenKind::Semicolon,
            // int c = a + b;
            TokenKind::Keyword(Keyword::Int),
            TokenKind::Ident("c".to_string()),
            TokenKind::Operator(Operator::Eq),
            TokenKind::Ident("a".to_string()),
            TokenKind::Operator(Operator::Plus),
            TokenKind::Ident("b".to_string()),
            TokenKind::Semicolon,
            // return 1 + 2 * 3 / 4;
            TokenKind::Keyword(Keyword::Return),
            TokenKind::Number(1),
            TokenKind::Operator(Operator::Plus),
            TokenKind::Number(2),
            TokenKind::Operator(Operator::Asterisk),
            TokenKind::Number(3),
            TokenKind::Operator(Operator::Slash),
            TokenKind::Number(4),
            TokenKind::Semicolon,
            TokenKind::RightBrace,
            TokenKind::Eof,
        ]
    );
}
