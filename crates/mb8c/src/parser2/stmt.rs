use chumsky::{prelude::just, select, Parser};

use crate::{
    parser::ast::Stmt,
    tokenizer::token::{Keyword, Operator, TokenKind},
};

use super::{expr::expr_parser, ty::ty_parser};

pub fn stmt_parser<'src>() -> impl Parser<'src, &'src [TokenKind], Stmt> {
    return_parser().or(declaration_parser())
}

pub fn return_parser<'src>() -> impl Parser<'src, &'src [TokenKind], Stmt> {
    just(TokenKind::Keyword(Keyword::Return))
        .ignore_then(expr_parser().or_not())
        .then_ignore(just(TokenKind::Semicolon))
        .map(|expr| Stmt::Return(expr))
}

pub fn declaration_parser<'src>() -> impl Parser<'src, &'src [TokenKind], Stmt> {
    ty_parser()
        .then(select! {TokenKind::Ident(name) => name})
        .then(
            just(TokenKind::Operator(Operator::Eq))
                .ignore_then(expr_parser())
                .or_not(),
        )
        .then_ignore(just(TokenKind::Semicolon))
        .map(|((ty, name), init)| Stmt::Declaration { name, ty, init })
}
