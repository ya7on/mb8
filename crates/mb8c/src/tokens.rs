use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum TokenKind {
    #[regex(r"[ \t\n\r]+", logos::skip)]
    #[regex(r"//[^\n]*", logos::skip, allow_greedy = true)]
    #[regex(r"/\*([^*]|\*+[^*/])*\*+/", logos::skip)]
    _Skip,

    /// Identifier (e.g. variable name)
    #[regex(r"[a-zA-Z]+", |lex| lex.slice().to_string())]
    Ident(String),
    /// Number (e.g. 123)
    #[regex("[0-9]+", |lex| lex.slice().parse::<i16>().ok())]
    Number(i16),

    /* Keywords */
    /// Void
    #[token("void")]
    KeywordVoid,
    /// Char. 8-bit type
    #[token("char")]
    KeywordChar,
    /// Integer. 16-bit type
    #[token("int")]
    KeywordInt,
    /// If
    #[token("if")]
    KeywordIf,
    /// Else
    #[token("else")]
    KeywordElse,
    /// While
    #[token("while")]
    KeywordWhile,
    /// Return
    #[token("return")]
    KeywordReturn,

    /* Operators */
    /// Plus +
    #[token("+")]
    OperatorPlus,
    /// Minus -
    #[token("-")]
    OperatorMinus,
    /// Asterisk *
    #[token("*")]
    OperatorAsterisk,
    /// Slash /
    #[token("/")]
    OperatorSlash,
    /// Equal =
    #[token("=")]
    OperatorEq,
    /// Equal equal ==
    #[token("==")]
    OperatorEqEq,

    /* Delimiters */
    /// Left brace {
    #[token("{")]
    LeftBrace,
    /// Right brace }
    #[token("}")]
    RightBrace,
    /// Left parenthesis (
    #[token("(")]
    LeftParenthesis,
    /// Right parenthesis )
    #[token(")")]
    RightParenthesis,
    /// Comma ,
    #[token(",")]
    Comma,
    /// Semicolon ;
    #[token(";")]
    Semicolon,
}
