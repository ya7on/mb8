use std::collections::HashMap;

use crate::{
    diagnostics::{Diagnostic, DiagnosticKind, Severity, Span},
    ir::{IRItem, IRProgram, TaggedProgram},
    pass::{AssemblerPass, PassContext},
};

pub(crate) struct TagPass;

impl AssemblerPass for TagPass {
    type Input = IRProgram;
    type Output = TaggedProgram;

    fn run(&mut self, input: Self::Input, context: &mut PassContext<'_>) -> Option<Self::Output> {
        let mut labels = HashMap::<String, (u16, Span)>::new();
        let mut counter = input.origin.as_ref().map_or(0, |origin| origin.value);

        for item in &input.items {
            match item {
                IRItem::Label(label) => {
                    if let Some((_, first)) =
                        labels.insert(label.value.clone(), (counter, label.span.clone()))
                    {
                        context.emit_fatal(Diagnostic {
                            severity: Severity::Error,
                            span: Some(label.span.clone()),
                            kind: DiagnosticKind::DuplicateLabel {
                                label: label
                                    .value
                                    .rsplit_once("::")
                                    .map_or(label.value.as_str(), |(_, local)| local)
                                    .to_string(),
                                first,
                            },
                        });
                        return None;
                    }
                }
                IRItem::Instruction(instruction) => {
                    let Some(address) = counter.checked_add(2) else {
                        context.emit_fatal(Diagnostic {
                            severity: Severity::Error,
                            span: Some(instruction.span.clone()),
                            kind: DiagnosticKind::AddressOverflow { current: counter },
                        });
                        return None;
                    };
                    counter = address;
                }
                IRItem::Data(data) => {
                    let Ok(size) = u16::try_from(data.value.len()) else {
                        context.emit_fatal(Diagnostic {
                            severity: Severity::Error,
                            span: Some(data.span.clone()),
                            kind: DiagnosticKind::AddressOverflow { current: counter },
                        });
                        return None;
                    };
                    let Some(address) = counter.checked_add(size) else {
                        context.emit_fatal(Diagnostic {
                            severity: Severity::Error,
                            span: Some(data.span.clone()),
                            kind: DiagnosticKind::AddressOverflow { current: counter },
                        });
                        return None;
                    };
                    counter = address;
                }
                IRItem::Address(address) => {
                    if address.value < counter {
                        context.emit_fatal(Diagnostic {
                            severity: Severity::Error,
                            span: Some(address.span.clone()),
                            kind: DiagnosticKind::InvalidAddressDirective {
                                current: counter,
                                target: address.value,
                            },
                        });
                        return None;
                    }
                    counter = address.value;
                }
            }
        }

        let labels = labels
            .into_iter()
            .map(|(label, (address, _))| (label, address))
            .collect();
        Some(TaggedProgram { ir: input, labels })
    }
}
