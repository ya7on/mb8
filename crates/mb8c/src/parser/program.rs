use chumsky::{
    error::Simple, extra::Err, input::ValueInput, prelude::end, span::SimpleSpan, IterParser,
    Parser,
};

use crate::{ast::ASTProgram, lex::tokens::TokenKind};

use super::{function::function_parser, globals::globals_parser};

#[must_use]
pub fn program_parser<'src, I>() -> impl Parser<'src, I, ASTProgram, Err<Simple<'src, TokenKind>>>
where
    I: ValueInput<'src, Token = TokenKind, Span = SimpleSpan>,
{
    globals_parser()
        .then(function_parser().repeated().collect())
        .then_ignore(end())
        .map(|(globals, functions)| ASTProgram { globals, functions })
}
