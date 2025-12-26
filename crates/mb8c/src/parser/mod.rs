use chumsky::{
    input::{Input, Stream},
    span::SimpleSpan,
    Parser as ChumskyParser,
};
use parse_program::program_parser;

use crate::{
    error::{CompileError, CompileResult},
    lex::tokens::TokenKind,
    parser::ast::ASTProgram,
    pipeline::CompilerPipe,
};

pub mod ast;
pub mod parse_expr;
pub mod parse_function;
pub mod parse_globals;
pub mod parse_program;
pub mod parse_stmt;
pub mod parse_ty;

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
