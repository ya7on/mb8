use crate::{
    ast::{ASTItem, ASTProgram, Directive},
    diagnostics::{Diagnostic, DiagnosticKind, Severity, Spanned},
    instructions::HANDLERS,
    ir::{IRItem, IRProgram},
    pass::{AssemblerPass, PassContext},
};

pub(crate) struct DesugarPass;

impl AssemblerPass for DesugarPass {
    type Input = ASTProgram;
    type Output = IRProgram;

    fn run(&mut self, input: Self::Input, context: &mut PassContext<'_>) -> Option<Self::Output> {
        let mut result = IRProgram {
            origin: None,
            items: Vec::new(),
        };

        for (id, item) in input.items.iter().enumerate() {
            match item {
                ASTItem::Instruction(instruction) => {
                    let Some(items) = HANDLERS
                        .iter()
                        .filter(|definition| definition.mnemonic == instruction.value.mnemonic)
                        .find_map(|definition| {
                            (definition.handler)(&instruction.value, &instruction.span, id)
                        })
                    else {
                        context.emit_fatal(Diagnostic {
                            severity: Severity::Error,
                            span: Some(instruction.span.clone()),
                            kind: DiagnosticKind::UnsupportedInstruction {
                                mnemonic: instruction.value.mnemonic.clone(),
                                operands: instruction.value.operands.clone(),
                            },
                        });
                        return None;
                    };
                    result.items.extend(items);
                }
                ASTItem::Label(label) => {
                    result.items.push(IRItem::Label(label.clone()));
                }
                ASTItem::Directive(directive) => match &directive.value {
                    Directive::Origin(address) => {
                        if let Some(origin) = &result.origin {
                            context.emit_fatal(Diagnostic {
                                severity: Severity::Error,
                                span: Some(directive.span.clone()),
                                kind: DiagnosticKind::DuplicateOrigin {
                                    first: origin.span.clone(),
                                },
                            });
                            return None;
                        }
                        result.origin = Some(Spanned {
                            value: *address,
                            span: directive.span.clone(),
                        });
                    }
                    Directive::Address(address) => {
                        result.items.push(IRItem::Address(Spanned {
                            value: *address,
                            span: directive.span.clone(),
                        }));
                    }
                    Directive::Data(bytes) => {
                        result.items.push(IRItem::Data(Spanned {
                            value: bytes.clone(),
                            span: directive.span.clone(),
                        }));
                    }
                    Directive::Ascii(text) => {
                        result.items.push(IRItem::Data(Spanned {
                            value: text.as_bytes().to_vec(),
                            span: directive.span.clone(),
                        }));
                    }
                    Directive::Include(_) => {
                        context.emit_fatal(Diagnostic {
                            severity: Severity::Error,
                            span: Some(directive.span.clone()),
                            kind: DiagnosticKind::UnexpectedDirective {
                                directive: directive.value.clone(),
                            },
                        });
                        return None;
                    }
                },
            }
        }

        Some(result)
    }
}
