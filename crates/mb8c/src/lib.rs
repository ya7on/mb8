use chumsky::Parser;
use codegen::CodeGenerator;
use error::CompileError;
use ir::lower_program;
use logos::Logos;
use parser::program::program_parser;
use tokens::TokenKind;

pub mod ast;
pub mod codegen;
pub mod error;
pub mod ir;
pub mod parser;
pub mod semantic;
pub mod tokens;

/// Compile the input string into an executable program.
///
/// # Errors
/// Returns an error if the input string is not valid MB8C code.
pub fn compile(input: &str) -> error::CompileResult<()> {
    let tokens = TokenKind::lexer(input)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|()| CompileError::InternalError {
            message: "Unknown error".to_owned(),
        })?;
    let parser = program_parser();
    let ast = parser
        .parse(&tokens)
        .into_result()
        .map_err(|_| CompileError::InternalError {
            message: "Unknown error".to_owned(),
        })?;

    semantic::analyze(&ast)?;

    let ir = lower_program(&ast)?;

    let code = CodeGenerator::new(ir).generate()?;
    println!("{code}");

    Ok(())
}
