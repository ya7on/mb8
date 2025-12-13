use chumsky::{error::Simple, extra::Err, prelude::end, IterParser, Parser};

use crate::{ast::ASTProgram, tokens::TokenKind};

use super::function::function_parser;

#[must_use]
pub fn program_parser<'src>(
) -> impl Parser<'src, &'src [TokenKind], ASTProgram, Err<Simple<'src, TokenKind>>> {
    function_parser()
        .repeated()
        .collect()
        .then_ignore(end())
        .map(|functions| ASTProgram { functions })
}
