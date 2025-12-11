use chumsky::{prelude::just, IterParser, Parser};

use crate::{parser::ast::Program, tokenizer::token::TokenKind};

use super::function::function_parser;

pub fn program_parser<'src>() -> impl Parser<'src, &'src [TokenKind], Program> {
    function_parser()
        .repeated()
        .collect()
        .then_ignore(just(TokenKind::Eof))
        .map(|functions| Program { functions })
}
