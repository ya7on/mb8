use codegen::targets::Codegen;
use error::CompileError;
use lower::handlers::Lower;
use parser::Parser;
use pipe::CompilePipeline;
use semantic::analysis::SemanticAnalysis;
use tokens::TokenKind;

pub mod ast;
pub mod codegen;
pub mod config;
pub mod error;
pub mod hir;
pub mod ir;
pub mod lower;
pub mod parser;
pub mod pipe;
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
    let result = CompilePipeline::<TokenKind>::init(input.to_owned())?
        .and_next::<Parser>()?
        .and_next::<SemanticAnalysis>()?
        .and_next::<Lower>()?
        .and_next::<Codegen>()?
        .finish()?;

    println!("{result}");

    Ok(())
}
