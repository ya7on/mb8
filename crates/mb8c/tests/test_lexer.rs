use mb8c::tokenizer::{
    lexer::Lexer,
    token::{Keyword, Operator, TokenKind},
};

#[test]
fn test_all_tokens() {
    let src = r#"
        // number
        10
        // identifier
        identifier
        // keywords
        int char void if else return
        // operators
        + - * / = ==
        // delimiters
        ( ) { } , ;
    "#;
    let result = Lexer::new(src).tokenize();
    assert_eq!(
        result
            .unwrap()
            .into_iter()
            .map(|token| token.kind)
            .collect::<Vec<_>>(),
        vec![
            // number
            TokenKind::Number(10),
            // identifier
            TokenKind::Ident("identifier".to_owned()),
            // keywords
            TokenKind::Keyword(Keyword::Int),
            TokenKind::Keyword(Keyword::Char),
            TokenKind::Keyword(Keyword::Void),
            TokenKind::Keyword(Keyword::If),
            TokenKind::Keyword(Keyword::Else),
            TokenKind::Keyword(Keyword::Return),
            // operators
            TokenKind::Operator(Operator::Plus),
            TokenKind::Operator(Operator::Minus),
            TokenKind::Operator(Operator::Asterisk),
            TokenKind::Operator(Operator::Slash),
            TokenKind::Operator(Operator::Eq),
            TokenKind::Operator(Operator::EqEq),
            // delimiters
            TokenKind::LeftParenthesis,
            TokenKind::RightParenthesis,
            TokenKind::LeftBrace,
            TokenKind::RightBrace,
            TokenKind::Comma,
            TokenKind::Semicolon,
            // eof
            TokenKind::Eof,
        ]
    );
}
