use chumsky::{prelude::end, IterParser, Parser};

use crate::{ast::Program, tokens::TokenKind};

use super::function::function_parser;

#[must_use]
pub fn program_parser<'src>() -> impl Parser<'src, &'src [TokenKind], Program> {
    function_parser()
        .repeated()
        .collect()
        .then_ignore(end())
        .map(|functions| Program { functions })
}
