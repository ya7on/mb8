use chumsky::{
    error::Simple, extra::Err, input::ValueInput, prelude::just, select, span::SimpleSpan,
    IterParser, Parser,
};

use crate::{
    ast::{ASTFunction, ASTStmt, Span},
    tokens::TokenKind,
};

use super::{stmt::stmt_parser, ty::ty_parser};

#[must_use]
pub fn function_parser<'src, I>() -> impl Parser<'src, I, ASTFunction, Err<Simple<'src, TokenKind>>>
where
    I: ValueInput<'src, Token = TokenKind, Span = SimpleSpan>,
{
    let param = select! { TokenKind::Ident(name) => name }
        .then_ignore(just(TokenKind::Colon))
        .then(ty_parser())
        .map(|(name, ty)| (name, ty));
    let params = param
        .separated_by(just(TokenKind::Comma))
        .collect()
        .delimited_by(
            just(TokenKind::LeftParenthesis),
            just(TokenKind::RightParenthesis),
        );

    let var = select! { TokenKind::Ident(name) => name }
        .then_ignore(just(TokenKind::Colon))
        .then(ty_parser())
        .then_ignore(just(TokenKind::Semicolon))
        .map_with(|(name, ty), extra| {
            let _span = extra.span();
            (name, ty)
        });
    let vars_declaration = just(TokenKind::KeywordVar)
        .ignore_then(var.repeated().collect::<Vec<_>>())
        .map_with(|vars, extra| {
            let _span = extra.span();
            vars
        });

    just(TokenKind::KeywordFunction)
        .ignore_then(select! { TokenKind::Ident(name) => name })
        .then(params)
        .then_ignore(just(TokenKind::Colon))
        .then(ty_parser())
        .then_ignore(just(TokenKind::Semicolon))
        .then(vars_declaration.or_not())
        .then(
            stmt_parser()
                .repeated()
                .collect()
                .delimited_by(just(TokenKind::KeywordBegin), just(TokenKind::KeywordEnd)),
        )
        .map_with(|((((name, params), return_type), vars), body), extra| {
            let span: SimpleSpan = extra.span();
            ASTFunction {
                name,
                params,
                return_type,
                vars: vars.unwrap_or_default(),
                body: ASTStmt::Block(body),
                span: Span {
                    start: span.start,
                    end: span.end,
                },
            }
        })
}
