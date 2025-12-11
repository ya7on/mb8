use chumsky::{prelude::todo, Parser};

use crate::{parser::ast::Expr, tokenizer::token::TokenKind};

pub fn expr_parser<'src>() -> impl Parser<'src, &'src [TokenKind], Expr> {
    todo()
}
