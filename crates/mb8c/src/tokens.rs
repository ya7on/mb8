use chumsky::span::SimpleSpan;
use logos::Logos;

use crate::{
    error::{CompileError, CompileResult},
    pipe::CompilerPipe,
};

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
    #[regex(r"[a-zA-Z_]+", |lex| lex.slice().to_string())]
    Ident(String),
    /// Number (e.g. 123)
    #[regex("[0-9]+", |lex| lex.slice().parse::<u16>().ok())]
    Number(u16),

    /* Keywords */
    /// Void
    #[token("void")]
    KeywordVoid,
    /// 8-bit unsigned integer
    #[token("u8")]
    KeywordU8,
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

impl CompilerPipe for TokenKind {
    type Prev = String;
    type Next = Vec<(TokenKind, SimpleSpan)>;

    fn execute(prev: &Self::Prev) -> CompileResult<Self::Next, Vec<CompileError>> {
        let mut result = Vec::new();

        for (token, span) in TokenKind::lexer(prev).spanned() {
            result.push((token.map_err(|err| vec![err])?, SimpleSpan::from(span)));
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokens() {
        let input = r"
        // comments
        // if then begin end -- ignored

        // ident
        name AbCdEf u_n_d_e_r_l_i_n_e
        // numer
        1 1337 228
        // negative
        -111

        // types
        void u8
        // keywords
        function var begin end
        if then
        while do
        return
        + - * / = ==
        ( ) , : ;
        ";

        let result = TokenKind::lexer(input)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(
            result,
            vec![
                TokenKind::Ident("name".to_string()),
                TokenKind::Ident("AbCdEf".to_string()),
                TokenKind::Ident("u_n_d_e_r_l_i_n_e".to_string()),
                TokenKind::Number(1),
                TokenKind::Number(1337),
                TokenKind::Number(228),
                TokenKind::OperatorMinus,
                TokenKind::Number(111),
                TokenKind::KeywordVoid,
                TokenKind::KeywordU8,
                TokenKind::KeywordFunction,
                TokenKind::KeywordVar,
                TokenKind::KeywordBegin,
                TokenKind::KeywordEnd,
                TokenKind::KeywordIf,
                TokenKind::KeywordThen,
                TokenKind::KeywordWhile,
                TokenKind::KeywordDo,
                TokenKind::KeywordReturn,
                TokenKind::OperatorPlus,
                TokenKind::OperatorMinus,
                TokenKind::OperatorAsterisk,
                TokenKind::OperatorSlash,
                TokenKind::OperatorEq,
                TokenKind::OperatorEqEq,
                TokenKind::LeftParenthesis,
                TokenKind::RightParenthesis,
                TokenKind::Comma,
                TokenKind::Colon,
                TokenKind::Semicolon
            ]
        );
    }
}
