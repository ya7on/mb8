#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct Span {
    /// Start position of the token in the source code.
    pub start: usize,
    /// End position of the token in the source code.
    pub end: usize,
    /// Line number where the token starts.
    pub line: usize,
    /// Column number where the token starts.
    pub column: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    /// Identifier (e.g. variable name)
    Ident(String),
    /// Number (e.g. 123)
    Number(i16),
    /// Reserved keyword (e.g. int)
    Keyword(Keyword),

    /// Operator (e.g. +, -, *, /)
    Operator(Operator),

    /* Delimiters */
    /// Left brace {
    LeftBrace,
    /// Right brace }
    RightBrace,
    /// Left parenthesis (
    LeftParenthesis,
    /// Right parenthesis )
    RightParenthesis,
    /// Comma ,
    Comma,
    /// Semicolon ;
    Semicolon,

    /// End of file
    Eof,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    /// Plus +
    Plus,
    /// Minus -
    Minus,
    /// Asterisk *
    Asterisk,
    /// Slash /
    Slash,
    /// Equal =
    Eq,

    /// Equal equal ==
    EqEq,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    /// Void
    Void,
    /// Char. 8-bit type
    Char,
    /// Integer. 16-bit type
    Int,
    /// If
    If,
    /// Else
    Else,
    /// While
    While,
    /// Return
    Return,
}
