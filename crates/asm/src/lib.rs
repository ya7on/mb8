mod ast;
mod desugar;
pub mod diagnostics;
pub mod error;
mod include;
mod ir;
mod lower;
mod parser;
mod tagging;
mod tokens;

use chumsky::{input::Input, span::SimpleSpan, Parser};
pub use diagnostics::SourceFile;
use diagnostics::Span;
pub use error::{AsmError, AsmErrorKind, AsmFailure};
use std::path::Path;

/// Compile an MB8 assembly file into encoded machine-code bytes.
///
/// # Errors
///
/// Returns an error when reading, lexing, parsing, include expansion, desugaring,
/// label tagging, or lowering fails.
pub fn compile_file(path: &Path) -> Result<Vec<u8>, AsmFailure> {
    let input = std::fs::read_to_string(path).map_err(|err| AsmFailure {
        error: AsmError {
            span: None,
            kind: AsmErrorKind::Include {
                message: format!("{}: {err}", path.display()),
            },
        },
        sources: vec![SourceFile {
            id: 0,
            name: path.display().to_string(),
            source: String::new(),
        }],
    })?;
    let base_dir = path.parent().unwrap_or_else(|| Path::new("."));
    compile_source(&input, path.display().to_string(), base_dir)
}

/// Compile MB8 assembly source into encoded machine-code bytes, resolving includes from `base_dir`.
///
/// # Errors
///
/// Returns an error when lexing, parsing, include expansion, desugaring, label tagging,
/// or lowering fails.
pub fn compile_source(
    input: &str,
    source_name: String,
    base_dir: &Path,
) -> Result<Vec<u8>, AsmFailure> {
    let mut sources = vec![SourceFile {
        id: 0,
        name: source_name,
        source: input.to_string(),
    }];

    let tokens = tokens::lexer()
        .parse(input)
        .into_result()
        .map_err(|errors| {
            let error = errors.first();
            AsmFailure {
                error: AsmError {
                    span: error.map(|error| Span {
                        source: 0,
                        range: error.span().into_range(),
                    }),
                    kind: AsmErrorKind::Lex {
                        message: error.map_or_else(
                            || "unknown lexer error".to_string(),
                            |error| format!("{error}"),
                        ),
                    },
                },
                sources: sources.clone(),
            }
        })?;
    let eoi = SimpleSpan::from(input.len()..input.len());
    let token_input = tokens.as_slice().split_token_span(eoi);
    let ast = parser::parser(0)
        .parse(token_input)
        .into_result()
        .map_err(|errors| {
            let error = errors.first();
            AsmFailure {
                error: AsmError {
                    span: error.map(|error| {
                        let span = *error.span();
                        Span {
                            source: 0,
                            range: span.start..span.end,
                        }
                    }),
                    kind: AsmErrorKind::Parse {
                        message: error.map_or_else(
                            || "unknown parser error".to_string(),
                            |error| format!("{error}"),
                        ),
                    },
                },
                sources: sources.clone(),
            }
        })?;
    let ast = include::expand(ast, base_dir, &mut sources).map_err(|error| AsmFailure {
        error,
        sources: sources.clone(),
    })?;
    let ir = desugar::desugar(&ast).map_err(|error| AsmFailure {
        error,
        sources: sources.clone(),
    })?;
    let labels = tagging::tag(&ir).map_err(|error| AsmFailure {
        error,
        sources: sources.clone(),
    })?;
    let bytes = lower::lower(&ir, &labels).map_err(|error| AsmFailure {
        error,
        sources: sources.clone(),
    })?;

    Ok(bytes)
}
