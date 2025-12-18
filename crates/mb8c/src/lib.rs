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

pub fn aaa(input: String) -> error::CompileResult<(), Vec<CompileError>> {
    let result = CompilePipeline::<TokenKind>::init(input)?
        .and_next::<Parser>()?
        .and_next::<SemanticAnalysis>()?
        .and_next::<Lower>()?;
    todo!()
}

/// Compile the input string into an executable program.
///
/// # Errors
/// Returns an error if the input string is not valid MB8C code.
///
/// # Panics
/// TODO
pub fn compile(input: &str) -> error::CompileResult<(), Vec<CompileError>> {
    // let tokens = TokenKind::lexer(input).spanned().map(|(tok, span)| {
    //     #[allow(clippy::expect_used)]
    //     (tok.expect("XYU"), SimpleSpan::from(span))
    // });
    // let token_stream =
    //     Stream::from_iter(tokens).map((0..input.len()).into(), |(t, s): (_, _)| (t, s));

    // let ast = program_parser()
    //     .parse(token_stream)
    //     .into_result()
    //     .map_err(|errors| {
    //         let mut result = Vec::with_capacity(errors.len());
    //         for err in errors {
    //             let span = err.span();
    //             let token = err.found();
    //             result.push(CompileError::ParserError {
    //                 start: span.start,
    //                 end: span.end,
    //                 found: token.cloned(),
    //             });
    //         }
    //         result
    //     })?;

    // let mut semantic_analyzer = SemanticAnalysis::default();
    // let hir = semantic_analyzer
    //     .analyze_program(&ast)
    //     .map_err(|err| vec![err])?;

    // let _ir = lower(&semantic_analyzer.ctx, &hir).map_err(|err| vec![err])?;

    // let code = Mb8Codegen::default()
    //     .codegen(&ir)
    //     .map_err(|err| vec![err])?;
    // println!("{code}");

    Ok(())
}
