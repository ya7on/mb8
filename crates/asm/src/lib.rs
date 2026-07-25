mod ast;
mod desugar;
pub mod diagnostics;
mod include;
mod instructions;
mod ir;
mod lints;
mod lower;
mod parser;
mod pass;
mod tagging;
mod tokens;

use std::path::Path;

use desugar::DesugarPass;
pub use diagnostics::{Diagnostic, DiagnosticKind, DiagnosticResult, Severity, SourceFile};
use include::IncludePass;
use lints::scratch_register::ScratchRegisterPass;
use lower::LowerPass;
use parser::ParsePass;
use tagging::TagPass;
use tokens::LexPass;

/// Compile an MB8 assembly file into encoded machine-code bytes.
#[must_use]
pub fn compile_file(path: &Path) -> DiagnosticResult<Vec<u8>> {
    let input = match std::fs::read_to_string(path) {
        Ok(input) => input,
        Err(error) => {
            return DiagnosticResult {
                result: None,
                diagnostics: vec![Diagnostic {
                    severity: Severity::Error,
                    span: None,
                    kind: DiagnosticKind::Include {
                        message: format!("{}: {error}", path.display()),
                    },
                }],
                ok: false,
                sources: vec![SourceFile {
                    id: 0,
                    name: path.display().to_string(),
                    source: String::new(),
                }],
            };
        }
    };
    let base_dir = path.parent().unwrap_or_else(|| Path::new("."));
    compile_source(&input, path.display().to_string(), base_dir)
}

/// Compile MB8 assembly source into encoded machine-code bytes, resolving includes from `base_dir`.
#[must_use]
pub fn compile_source(
    input: &str,
    source_name: String,
    base_dir: &Path,
) -> DiagnosticResult<Vec<u8>> {
    let sources = vec![SourceFile {
        id: 0,
        name: source_name,
        source: input.to_string(),
    }];

    DiagnosticResult {
        result: Some(0),
        diagnostics: Vec::new(),
        ok: true,
        sources,
    }
    .then(LexPass)
    .then(ParsePass)
    .then(IncludePass { base_dir })
    .then(ScratchRegisterPass)
    .then(DesugarPass)
    .then(TagPass)
    .then(LowerPass)
    .finalize()
}
