use chumsky::Parser;
use error::CompileError;
use logos::Logos;
use parser::program::program_parser;
use tokens::TokenKind;

pub mod ast;
pub mod codegen;
pub mod error;
pub mod hir;
pub mod parser;
pub mod semantic;
pub mod tokens;

/// Compile the input string into an executable program.
///
/// # Errors
/// Returns an error if the input string is not valid MB8C code.
pub fn compile(input: &str) -> error::CompileResult<(), Vec<CompileError>> {
    let tokens = TokenKind::lexer(input)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| vec![err])?;
    let parser = program_parser();
    let ast = parser.parse(&tokens).into_result().map_err(|errors| {
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

    println!("{hir:?}");

    Ok(())

    // let ir = lower_program(&ast)?;

    // let code = CodeGenerator::new(ir).generate()?;
    // println!("{code}");

    // Ok(())
}
