use chumsky::{error::Simple, extra::Err, prelude::just, select, IterParser, Parser};

use crate::{
    ast::{ASTFunction, ASTStmt, Span},
    tokens::TokenKind,
};

use super::{stmt::stmt_parser, ty::ty_parser};

#[must_use]
pub fn function_parser<'src>(
) -> impl Parser<'src, &'src [TokenKind], ASTFunction, Err<Simple<'src, TokenKind>>> {
    let param = ty_parser()
        .then(select! { TokenKind::Ident(name) => name })
        .map(|(ty, name)| (name, ty));
    let params = param
        .separated_by(just(TokenKind::Comma))
        .collect()
        .delimited_by(
            just(TokenKind::LeftParenthesis),
            just(TokenKind::RightParenthesis),
        );

    ty_parser()
        .then(select! { TokenKind::Ident(name) => name })
        .then(params)
        .then_ignore(just(TokenKind::LeftBrace))
        .then(stmt_parser().repeated().collect())
        .then_ignore(just(TokenKind::RightBrace))
        .map_with(|(((return_type, name), params), body), extra| {
            let span = extra.span();
            ASTFunction {
                return_type,
                name,
                params,
                body: ASTStmt::Block(body),
                span: Span {
                    start: span.start,
                    end: span.end,
                },
            }
        })
}
