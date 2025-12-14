use chumsky::{
    error::Simple, extra::Err, input::ValueInput, prelude::end, span::SimpleSpan, IterParser,
    Parser,
};

use crate::{ast::ASTProgram, tokens::TokenKind};

use super::function::function_parser;

#[must_use]
pub fn program_parser<'src, I>() -> impl Parser<'src, I, ASTProgram, Err<Simple<'src, TokenKind>>>
where
    I: ValueInput<'src, Token = TokenKind, Span = SimpleSpan>,
{
    function_parser()
        .repeated()
        .collect()
        .then_ignore(end())
        .map(|functions| ASTProgram { functions })
}
