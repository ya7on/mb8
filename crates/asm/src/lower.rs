use std::collections::HashMap;

use mb8_isa::{encode::encode, opcodes::Opcode};

use crate::{
    diagnostics::{Diagnostic, DiagnosticKind, Severity, Spanned},
    ir::{Expression, IRInstruction, IRItem, RelativeOffset, TaggedProgram},
    pass::{AssemblerPass, PassContext},
};

fn lower_expression(expr: &Expression, labels: &HashMap<String, u16>) -> Result<u16, Diagnostic> {
    match expr {
        Expression::Label(label) => labels.get(&label.value).copied().ok_or_else(|| Diagnostic {
            severity: Severity::Error,
            span: Some(label.span.clone()),
            kind: DiagnosticKind::UnknownLabel {
                label: label
                    .value
                    .rsplit_once("::")
                    .map_or(label.value.as_str(), |(_, local)| local)
                    .to_string(),
            },
        }),
        Expression::Immediate(value) => Ok(*value),
        Expression::Hi(expr) => Ok(lower_expression(expr, labels)? >> 8),
        Expression::Lo(expr) => Ok(lower_expression(expr, labels)? & 0x00FF),
    }
}

fn lower_instruction(
    instruction: &Spanned<IRInstruction>,
    labels: &HashMap<String, u16>,
    current_address: u16,
) -> Result<u16, Diagnostic> {
    let opcode = match &instruction.value {
        IRInstruction::Nop => Opcode::Nop,
        IRInstruction::Sys => Opcode::Sys,

        IRInstruction::Mov { dst, src } => Opcode::Mov {
            dst: *dst,
            src: *src,
        },
        IRInstruction::Add { dst, src } => Opcode::Add {
            dst: *dst,
            src: *src,
        },
        IRInstruction::Sub { dst, src } => Opcode::Sub {
            dst: *dst,
            src: *src,
        },
        IRInstruction::And { dst, src } => Opcode::And {
            dst: *dst,
            src: *src,
        },
        IRInstruction::Or { dst, src } => Opcode::Or {
            dst: *dst,
            src: *src,
        },
        IRInstruction::Xor { dst, src } => Opcode::Xor {
            dst: *dst,
            src: *src,
        },
        IRInstruction::Shr { dst, src } => Opcode::Shr {
            dst: *dst,
            src: *src,
        },
        IRInstruction::Shl { dst, src } => Opcode::Shl {
            dst: *dst,
            src: *src,
        },
        IRInstruction::Cmp { dst, src } => Opcode::Cmp {
            dst: *dst,
            src: *src,
        },

        IRInstruction::Ldi { dst, src } => {
            let value = lower_expression(src, labels)?;
            Opcode::Ldi {
                dst: *dst,
                value: u8::try_from(value).map_err(|_| Diagnostic {
                    severity: Severity::Error,
                    span: Some(instruction.span.clone()),
                    kind: DiagnosticKind::ValueOutOfRange {
                        value,
                        expected: "u8",
                    },
                })?,
            }
        }

        IRInstruction::Jmp { hi, lo } => Opcode::Jmp { hi: *hi, lo: *lo },
        IRInstruction::Jr { offset } => Opcode::Jr {
            offset: lower_relative_offset(offset, labels, current_address, &instruction.span)?,
        },
        IRInstruction::Jzr { offset } => Opcode::Jzr {
            offset: lower_relative_offset(offset, labels, current_address, &instruction.span)?,
        },
        IRInstruction::Jnzr { offset } => Opcode::Jnzr {
            offset: lower_relative_offset(offset, labels, current_address, &instruction.span)?,
        },
        IRInstruction::Jcr { offset } => Opcode::Jcr {
            offset: lower_relative_offset(offset, labels, current_address, &instruction.span)?,
        },
        IRInstruction::Jncr { offset } => Opcode::Jncr {
            offset: lower_relative_offset(offset, labels, current_address, &instruction.span)?,
        },

        IRInstruction::Call { hi, lo } => Opcode::Call { hi: *hi, lo: *lo },
        IRInstruction::Ret => Opcode::Ret,
        IRInstruction::Push { src } => Opcode::Push { src: *src },
        IRInstruction::Pop { dst } => Opcode::Pop { dst: *dst },
        IRInstruction::Ld { dst, hi, lo } => Opcode::Ld {
            dst: *dst,
            hi: *hi,
            lo: *lo,
        },
        IRInstruction::St { src, hi, lo } => Opcode::St {
            src: *src,
            hi: *hi,
            lo: *lo,
        },

        IRInstruction::Halt { code } => return Ok(0x0100 | u16::from(*code)),
    };

    Ok(encode(&opcode))
}

fn lower_relative_offset(
    offset: &RelativeOffset,
    labels: &HashMap<String, u16>,
    current_address: u16,
    span: &crate::diagnostics::Span,
) -> Result<i8, Diagnostic> {
    match offset {
        RelativeOffset::Immediate(value) => {
            let value = u8::try_from(*value).map_err(|_| Diagnostic {
                severity: Severity::Error,
                span: Some(span.clone()),
                kind: DiagnosticKind::ValueOutOfRange {
                    value: *value,
                    expected: "u8",
                },
            })?;
            Ok(value as i8)
        }
        RelativeOffset::Address(address) => {
            let target = i32::from(lower_expression(address, labels)?);
            let offset = target - i32::from(current_address) - 2;
            i8::try_from(offset).map_err(|_| Diagnostic {
                severity: Severity::Error,
                span: Some(span.clone()),
                kind: DiagnosticKind::RelativeJumpOutOfRange { offset },
            })
        }
    }
}

pub(crate) struct LowerPass;

impl AssemblerPass for LowerPass {
    type Input = TaggedProgram;
    type Output = Vec<u8>;

    fn run(&mut self, input: Self::Input, context: &mut PassContext<'_>) -> Option<Self::Output> {
        let mut result = Vec::new();
        let mut current_address = input.ir.origin.as_ref().map_or(0, |origin| origin.value);

        for item in &input.ir.items {
            match item {
                IRItem::Instruction(instruction) => {
                    let encoded =
                        match lower_instruction(instruction, &input.labels, current_address) {
                            Ok(encoded) => encoded,
                            Err(diagnostic) => {
                                context.emit_fatal(diagnostic);
                                return None;
                            }
                        };
                    result.extend(encoded.to_be_bytes());

                    let Some(address) = current_address.checked_add(2) else {
                        context.emit_fatal(Diagnostic {
                            severity: Severity::Error,
                            span: Some(instruction.span.clone()),
                            kind: DiagnosticKind::AddressOverflow {
                                current: current_address,
                            },
                        });
                        return None;
                    };
                    current_address = address;
                }
                IRItem::Data(data) => {
                    result.extend(&data.value);
                    let Ok(size) = u16::try_from(data.value.len()) else {
                        context.emit_fatal(Diagnostic {
                            severity: Severity::Error,
                            span: Some(data.span.clone()),
                            kind: DiagnosticKind::AddressOverflow {
                                current: current_address,
                            },
                        });
                        return None;
                    };
                    let Some(address) = current_address.checked_add(size) else {
                        context.emit_fatal(Diagnostic {
                            severity: Severity::Error,
                            span: Some(data.span.clone()),
                            kind: DiagnosticKind::AddressOverflow {
                                current: current_address,
                            },
                        });
                        return None;
                    };
                    current_address = address;
                }
                IRItem::Address(address) => {
                    let Some(padding) = address.value.checked_sub(current_address) else {
                        context.emit_fatal(Diagnostic {
                            severity: Severity::Error,
                            span: Some(address.span.clone()),
                            kind: DiagnosticKind::InvalidAddressDirective {
                                current: current_address,
                                target: address.value,
                            },
                        });
                        return None;
                    };
                    result.resize(result.len() + usize::from(padding), 0);
                    current_address = address.value;
                }
                IRItem::Label(_) => {}
            }
        }

        Some(result)
    }
}
