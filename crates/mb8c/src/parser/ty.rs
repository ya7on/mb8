use chumsky::{error::Simple, extra::Err, input::ValueInput, select, span::SimpleSpan, Parser};

use crate::{ast::ASTType, tokens::TokenKind};

#[must_use]
pub fn ty_parser<'src, I>() -> impl Parser<'src, I, ASTType, Err<Simple<'src, TokenKind>>> + Clone
where
    I: ValueInput<'src, Token = TokenKind, Span = SimpleSpan>,
{
    select! {
        TokenKind::KeywordVoid => ASTType::Void,
        TokenKind::KeywordU8 => ASTType::Unsigned8,
    }
}
