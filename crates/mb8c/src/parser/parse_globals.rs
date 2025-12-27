use chumsky::{
    error::Simple, extra::Err, input::ValueInput, prelude::just, select, span::SimpleSpan,
    IterParser, Parser,
};

use crate::{
    lex::tokens::TokenKind,
    parser::ast::{ASTGlobal, Span},
};

use super::parse_ty::ty_parser;

#[must_use]
pub fn globals_parser<'src, I>(
) -> impl Parser<'src, I, Vec<ASTGlobal>, Err<Simple<'src, TokenKind>>>
where
    I: ValueInput<'src, Token = TokenKind, Span = SimpleSpan>,
{
    let var = select! { TokenKind::Ident(name) => name }
        .then_ignore(just(TokenKind::Colon))
        .then(ty_parser())
        .then_ignore(just(TokenKind::At))
        .then(select! { TokenKind::LiteralU16(address) => address })
        .then_ignore(just(TokenKind::Semicolon))
        .map_with(|((name, ty), at), extra| {
            let span: SimpleSpan = extra.span();
            ASTGlobal {
                name,
                ty,
                at,
                span: Span {
                    start: span.start,
                    end: span.end,
                },
            }
        });

    just(TokenKind::KeywordVar)
        .ignore_then(var.repeated().collect())
        .or_not()
        .map_with(|vars, extra| {
            let _span = extra.span();
            vars.unwrap_or_default()
        })
}
