use logos::Logos;
use mb8c::tokens::TokenKind;

#[test]
fn test_all_tokens() {
    let src = r"
        /* ignored comment */
        // number
        10
        // identifier
        identifier
        // keywords
        int char void if else while return
        // operators
        + - * / = ==
        // delimiters
        ( ) { } , ;
    ";
    let result = TokenKind::lexer(src)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(
        result,
        vec![
            // number
            TokenKind::Number(10),
            // identifier
            TokenKind::Ident("identifier".to_owned()),
            // keywords
            TokenKind::KeywordInt,
            TokenKind::KeywordChar,
            TokenKind::KeywordVoid,
            TokenKind::KeywordIf,
            TokenKind::KeywordElse,
            TokenKind::KeywordWhile,
            TokenKind::KeywordReturn,
            // operators
            TokenKind::OperatorPlus,
            TokenKind::OperatorMinus,
            TokenKind::OperatorAsterisk,
            TokenKind::OperatorSlash,
            TokenKind::OperatorEq,
            TokenKind::OperatorEqEq,
            // delimiters
            TokenKind::LeftParenthesis,
            TokenKind::RightParenthesis,
            TokenKind::LeftBrace,
            TokenKind::RightBrace,
            TokenKind::Comma,
            TokenKind::Semicolon,
        ]
    );
}
