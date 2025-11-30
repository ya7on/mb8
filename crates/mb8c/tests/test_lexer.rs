use mb8c::tokenizer::{
    lexer::Lexer,
    token::{Keyword, Operator, TokenKind},
};

#[test]
fn test_main() {
    let src = r#"
        int main(int a, int b) {
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
            TokenKind::Keyword(Keyword::Int),
            TokenKind::Ident("main".to_string()),
            TokenKind::LeftParenthesis,
            TokenKind::Keyword(Keyword::Int),
            TokenKind::Ident("a".to_string()),
            TokenKind::Comma,
            TokenKind::Keyword(Keyword::Int),
            TokenKind::Ident("b".to_string()),
            TokenKind::RightParenthesis,
            TokenKind::LeftBrace,
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
