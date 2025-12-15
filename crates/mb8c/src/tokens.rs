use logos::Logos;

use crate::error::CompileError;

fn map_err(err: &mut logos::Lexer<TokenKind>) -> CompileError {
    let span = err.span();
    CompileError::UnexpectedToken {
        start: span.start,
        end: span.end,
    }
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(error(CompileError, map_err))]
pub enum TokenKind {
    #[regex(r"[ \t\n\r]+", logos::skip)]
    #[regex(r"//[^\n]*", logos::skip, allow_greedy = true)]
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
    /// 8-bit unsigned integer
    #[token("u8")]
    KeywordU8,
    /// program
    #[token("program")]
    KeywordProgram,
    /// function
    #[token("function")]
    KeywordFunction,
    /// var
    #[token("var")]
    KeywordVar,
    /// begin
    #[token("begin")]
    KeywordBegin,
    /// end
    #[token("end")]
    KeywordEnd,
    /// If
    #[token("if")]
    KeywordIf,
    /// Then
    #[token("then")]
    KeywordThen,
    /// Else
    #[token("else")]
    KeywordElse,
    /// While
    #[token("while")]
    KeywordWhile,
    /// Do
    #[token("do")]
    KeywordDo,
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
    /// Left parenthesis (
    #[token("(")]
    LeftParenthesis,
    /// Right parenthesis )
    #[token(")")]
    RightParenthesis,
    /// Comma ,
    #[token(",")]
    Comma,
    /// Colon ;
    #[token(":")]
    Colon,
    /// Semicolon ;
    #[token(";")]
    Semicolon,
}
