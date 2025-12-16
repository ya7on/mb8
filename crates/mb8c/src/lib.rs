use chumsky::{
    input::{Input, Stream},
    span::SimpleSpan,
    Parser,
};
use error::CompileError;
use logos::Logos;
use lower::lower;
use parser::program::program_parser;
use tokens::TokenKind;

pub mod ast;
pub mod codegen;
pub mod error;
pub mod hir;
pub mod ir;
pub mod lower;
pub mod parser;
pub mod semantic;
pub mod tokens;

/// Compile the input string into an executable program.
///
/// # Errors
/// Returns an error if the input string is not valid MB8C code.
///
/// # Panics
/// TODO
pub fn compile(input: &str) -> error::CompileResult<(), Vec<CompileError>> {
    let tokens = TokenKind::lexer(input).spanned().map(|(tok, span)| {
        #[allow(clippy::expect_used)]
        (tok.expect("XYU"), SimpleSpan::from(span))
    });
    let token_stream =
        Stream::from_iter(tokens).map((0..input.len()).into(), |(t, s): (_, _)| (t, s));

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

    let hir = semantic::analyze(&ast).map_err(|err| vec![err])?;

    lower(&hir).map_err(|err| vec![err])?;

    Ok(())

    // let ir = lower_program(&ast)?;

    // let code = CodeGenerator::new(ir).generate()?;
    // println!("{code}");

    // Ok(())
}
