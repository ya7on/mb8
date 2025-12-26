use codegen::targets::Codegen;
use error::CompileError;
use hir::lower::SemanticAnalysis;
use ir::lower::Lower;
use lex::tokens::TokenKind;
use parser::Parser;
use pipeline::CompilePipeline;

pub mod codegen;
pub mod config;
pub mod error;
pub mod hir;
pub mod ir;
pub mod lex;
pub mod parser;
pub mod pipeline;

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
