use std::collections::HashMap;

use mb8_isa::{encode::encode, opcodes::Opcode};

use crate::{
    diagnostics::Spanned,
    error::{AsmError, AsmErrorKind},
    ir::{Expression, IRInstruction, IRItem, IRProgram, RelativeOffset},
};

pub fn lower_expression(expr: &Expression, labels: &HashMap<String, u16>) -> Result<u16, AsmError> {
    match expr {
        Expression::Label(label) => labels.get(&label.value).copied().ok_or_else(|| AsmError {
            span: Some(label.span.clone()),
            kind: AsmErrorKind::UnknownLabel {
                label: label.value.clone(),
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
) -> Result<u16, AsmError> {
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
                value: u8::try_from(value).map_err(|_| AsmError {
                    span: Some(instruction.span.clone()),
                    kind: AsmErrorKind::ValueOutOfRange {
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
) -> Result<i8, AsmError> {
    match offset {
        RelativeOffset::Immediate(value) => {
            let value = u8::try_from(*value).map_err(|_| AsmError {
                span: Some(span.clone()),
                kind: AsmErrorKind::ValueOutOfRange {
                    value: *value,
                    expected: "u8",
                },
            })?;
            Ok(value as i8)
        }
        RelativeOffset::Address(address) => {
            let target = i32::from(lower_expression(address, labels)?);
            let offset = target - i32::from(current_address) - 2;
            i8::try_from(offset).map_err(|_| AsmError {
                span: Some(span.clone()),
                kind: AsmErrorKind::RelativeJumpOutOfRange { offset },
            })
        }
    }
}

pub fn lower(ir: &IRProgram, labels: &HashMap<String, u16>) -> Result<Vec<u8>, AsmError> {
    let mut result = Vec::new();
    let mut current_address = ir.origin.as_ref().map_or(0, |origin| origin.value);
    for item in &ir.items {
        match item {
            IRItem::Instruction(instr) => {
                result.extend(lower_instruction(instr, labels, current_address)?.to_be_bytes());
                current_address = current_address.checked_add(2).ok_or_else(|| AsmError {
                    span: Some(instr.span.clone()),
                    kind: AsmErrorKind::AddressOverflow {
                        current: current_address,
                    },
                })?;
            }
            IRItem::Data(data) => {
                result.extend(&data.value);
                current_address = current_address
                    .checked_add(u16::try_from(data.value.len()).map_err(|_| AsmError {
                        span: Some(data.span.clone()),
                        kind: AsmErrorKind::AddressOverflow {
                            current: current_address,
                        },
                    })?)
                    .ok_or_else(|| AsmError {
                        span: Some(data.span.clone()),
                        kind: AsmErrorKind::AddressOverflow {
                            current: current_address,
                        },
                    })?;
            }
            IRItem::Label(_) => {}
        }
    }
    Ok(result)
}
