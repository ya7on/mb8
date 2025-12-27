use codegen::targets::mb8::Mb8Codegen;
use error::CompileError;
use hir::lower::HIRLowerer;
use ir::lower::IRLowerer;
use layout::pass::LayoutPass;
use lex::tokens::TokenKind;
use parser::Parser;
use pipeline::CompilePipeline;

pub mod codegen;
pub mod config;
pub mod context;
pub mod error;
pub mod hir;
pub mod ir;
pub mod layout;
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
        .and_next::<HIRLowerer>()?
        .and_next::<IRLowerer>()?
        .and_next::<LayoutPass>()?
        .and_next::<Mb8Codegen>()?
        .finish()?;

    println!("{result}");

    Ok(())
}
