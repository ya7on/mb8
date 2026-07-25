use std::path::Path;

use crate::{
    ast::{ASTItem, ASTProgram, Directive},
    diagnostics::{Diagnostic, DiagnosticKind, Severity, Spanned},
    parser::ParsePass,
    pass::{AssemblerPass, PassContext},
    tokens::LexPass,
};

pub(crate) struct IncludePass<'a> {
    pub base_dir: &'a Path,
}

impl AssemblerPass for IncludePass<'_> {
    type Input = ASTProgram;
    type Output = ASTProgram;

    fn run(&mut self, input: Self::Input, context: &mut PassContext<'_>) -> Option<Self::Output> {
        let mut items = Vec::new();

        for item in input.items {
            match item {
                ASTItem::Directive(directive) => match directive.value {
                    Directive::Include(include_path) => {
                        let path = self.base_dir.join(include_path);
                        let source = match std::fs::read_to_string(&path) {
                            Ok(source) => source,
                            Err(error) => {
                                context.emit_fatal(Diagnostic {
                                    severity: Severity::Error,
                                    span: Some(directive.span.clone()),
                                    kind: DiagnosticKind::Include {
                                        message: format!("{}: {error}", path.display()),
                                    },
                                });
                                return None;
                            }
                        };
                        let source_id = context.add_source(path.display().to_string(), source);
                        let tokens = LexPass.run(source_id, context)?;
                        let ast = ParsePass.run(tokens, context)?;
                        let base_dir = path.parent().unwrap_or(self.base_dir);
                        let expanded = IncludePass { base_dir }.run(ast, context)?;
                        items.extend(expanded.items);
                    }
                    value => items.push(ASTItem::Directive(Spanned {
                        value,
                        span: directive.span,
                    })),
                },
                item => items.push(item),
            }
        }

        Some(ASTProgram { items })
    }
}
