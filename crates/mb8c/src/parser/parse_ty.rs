use chumsky::{
    error::Simple,
    extra::Err,
    input::ValueInput,
    prelude::{just, recursive},
    select,
    span::SimpleSpan,
    Parser,
};

use crate::{lex::tokens::TokenKind, parser::ast::ASTType};

#[must_use]
pub fn ty_parser<'src, I>() -> impl Parser<'src, I, ASTType, Err<Simple<'src, TokenKind>>> + Clone
where
    I: ValueInput<'src, Token = TokenKind, Span = SimpleSpan>,
{
    recursive(|ty| {
        select! {
            TokenKind::KeywordVoid => ASTType::Void,
            TokenKind::KeywordU8 => ASTType::Unsigned8,
            TokenKind::KeywordU16 => ASTType::Unsigned16,
        }
        .or(just(TokenKind::OperatorAsterisk)
            .ignore_then(ty.clone())
            .map(|inner| ASTType::Pointer(Box::new(inner))))
    })
}
