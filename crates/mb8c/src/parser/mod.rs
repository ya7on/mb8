use chumsky::{
    input::{Input, Stream},
    span::SimpleSpan,
    Parser as ChumskyParser,
};
use program::program_parser;

use crate::{
    ast::ASTProgram,
    error::{CompileError, CompileResult},
    pipe::CompilerPipe,
    tokens::TokenKind,
};

pub mod expr;
pub mod function;
pub mod program;
pub mod stmt;
pub mod ty;

#[derive(Debug)]
pub struct Parser {}

impl CompilerPipe for Parser {
    type Prev = Vec<(TokenKind, SimpleSpan)>;
    type Next = ASTProgram;

    fn execute(prev: &Self::Prev) -> CompileResult<Self::Next, Vec<CompileError>> {
        let token_stream =
            Stream::from_iter(prev.to_owned()).map((0..prev.len()).into(), |(t, s): (_, _)| (t, s));
        let ast = program_parser()
            .parse(token_stream)
            .into_result()
            .map_err(|errors| {
                let mut result = Vec::with_capacity(errors.len());
                for err in errors {
                    let span = err.span();
                    let token = err.found();
                    result.push(CompileError::ParserError {
                        start: span.start,
                        end: span.end,
                        found: token.cloned(),
                    });
                }
                result
            })?;
        Ok(ast)
    }
}
