use std::path::Path;

use chumsky::{input::Input, span::SimpleSpan, Parser};

use crate::{
    ast::{ASTItem, ASTProgram, Directive},
    diagnostics::{SourceFile, Span, Spanned},
    error::{AsmError, AsmErrorKind},
    parser::parser,
    tokens::lexer,
};

pub fn expand(
    ast: ASTProgram,
    base_dir: &Path,
    sources: &mut Vec<SourceFile>,
) -> Result<ASTProgram, AsmError> {
    let mut items = Vec::new();

    for item in ast.items {
        match item {
            ASTItem::Directive(directive) => {
                match directive.value {
                    Directive::Include(include_path) => {
                        let path = base_dir.join(include_path);
                        let source = std::fs::read_to_string(&path).map_err(|err| AsmError {
                            span: Some(directive.span.clone()),
                            kind: AsmErrorKind::Include {
                                message: format!("{}: {err}", path.display()),
                            },
                        })?;
                        let source_id = sources.len();
                        sources.push(SourceFile {
                            id: source_id,
                            name: path.display().to_string(),
                            source: source.clone(),
                        });
                        let tokens =
                            lexer()
                                .parse(source.as_str())
                                .into_result()
                                .map_err(|errors| {
                                    let error = errors.first();
                                    AsmError {
                                        span: error.map(|error| Span {
                                            source: source_id,
                                            range: error.span().into_range(),
                                        }),
                                        kind: AsmErrorKind::Lex {
                                            message: error.map_or_else(
                                                || "unknown lexer error".to_string(),
                                                |error| format!("{error}"),
                                            ),
                                        },
                                    }
                                })?;
                        let eoi = SimpleSpan::from(source.len()..source.len());
                        let token_input = tokens.as_slice().split_token_span(eoi);
                        let ast = parser(source_id).parse(token_input).into_result().map_err(
                            |errors| {
                                let error = errors.first();
                                AsmError {
                                    span: error.map(|error| {
                                        let span = *error.span();
                                        Span {
                                            source: source_id,
                                            range: span.start..span.end,
                                        }
                                    }),
                                    kind: AsmErrorKind::Parse {
                                        message: error.map_or_else(
                                            || "unknown parser error".to_string(),
                                            |error| format!("{error}"),
                                        ),
                                    },
                                }
                            },
                        )?;
                        let base_dir = path.parent().unwrap_or(base_dir);
                        items.extend(expand(ast, base_dir, sources)?.items);
                    }
                    value => items.push(ASTItem::Directive(Spanned {
                        value,
                        span: directive.span,
                    })),
                }
            }
            item => items.push(item),
        }
    }

    Ok(ASTProgram { items })
}
