use std::collections::HashMap;

use crate::{
    ast::{ASTItem, ASTProgram, DataSource, Directive, Operand},
    diagnostics::{Diagnostic, DiagnosticKind, Severity, Span, Spanned},
    pass::{AssemblerPass, PassContext},
};

pub(crate) struct ConstPass;

impl AssemblerPass for ConstPass {
    type Input = ASTProgram;
    type Output = ASTProgram;

    fn run(&mut self, input: Self::Input, context: &mut PassContext<'_>) -> Option<Self::Output> {
        let mut constants = HashMap::<String, (u16, Span)>::new();

        for item in &input.items {
            let ASTItem::Directive(Spanned {
                value: Directive::Const { name, value },
                ..
            }) = item
            else {
                continue;
            };

            if let Some((_, first)) =
                constants.insert(name.value.clone(), (*value, name.span.clone()))
            {
                context.emit_fatal(Diagnostic {
                    severity: Severity::Error,
                    span: Some(name.span.clone()),
                    kind: DiagnosticKind::DuplicateConstant {
                        name: name.value.clone(),
                        first,
                    },
                });
                return None;
            }
        }

        let mut items = Vec::with_capacity(input.items.len());
        for mut item in input.items {
            match &mut item {
                ASTItem::Instruction(instruction) => {
                    for operand in &mut instruction.value.operands {
                        match operand {
                            Operand::Raw(source) | Operand::MemoryWrapped(source) => {
                                let DataSource::Constant(constant) = source else {
                                    continue;
                                };
                                let Some((value, _)) = constants.get(&constant.value) else {
                                    context.emit_fatal(Diagnostic {
                                        severity: Severity::Error,
                                        span: Some(constant.span.clone()),
                                        kind: DiagnosticKind::UnknownConstant {
                                            name: constant.value.clone(),
                                        },
                                    });
                                    return None;
                                };
                                *source = DataSource::Immediate(*value);
                            }
                            Operand::MemoryOffset { .. } => {}
                        }
                    }
                }
                ASTItem::Directive(directive) => {
                    if matches!(directive.value, Directive::Const { .. }) {
                        continue;
                    }
                }
                ASTItem::Label(_) => {}
            }
            items.push(item);
        }

        Some(ASTProgram { items })
    }
}
