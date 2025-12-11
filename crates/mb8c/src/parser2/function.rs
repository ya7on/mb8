use chumsky::{prelude::just, select, IterParser, Parser};

use crate::{parser::ast::Function, tokenizer::token::TokenKind};

use super::{stmt::stmt_parser, ty::ty_parser};

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
        .then(stmt_parser())
        .map(|(((return_type, name), params), body)| Function {
            return_type,
            name,
            params,
            body,
        })
}
