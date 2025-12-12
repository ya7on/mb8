use chumsky::{prelude::just, select, IterParser, Parser};

use crate::{
    ast::{Function, Stmt},
    tokens::TokenKind,
};

use super::{stmt::stmt_parser, ty::ty_parser};

#[must_use]
pub fn function_parser<'src>() -> impl Parser<'src, &'src [TokenKind], Function> {
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
        .map(|(((return_type, name), params), body)| Function {
            return_type,
            name,
            params,
            body: Stmt::Block(body),
        })
}
