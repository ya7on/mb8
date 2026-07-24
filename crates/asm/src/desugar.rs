use mb8_isa::registers::Register;

use crate::{
    ast::{ASTInstruction, ASTItem, ASTProgram, DataSource, Directive, Operand},
    diagnostics::{Span, Spanned},
    error::{AsmError, AsmErrorKind},
    ir::{Expression, IRInstruction, IRItem, IRProgram, RelativeOffset},
};

pub fn desugar_instruction(
    instruction: &Spanned<ASTInstruction>,
    id: usize,
) -> Result<Vec<IRItem>, AsmError> {
    desugar_system(&instruction.value, &instruction.span)
        .or_else(|| desugar_reg_reg(&instruction.value, &instruction.span))
        .or_else(|| desugar_ldi(&instruction.value, &instruction.span))
        .or_else(|| desugar_jmp(&instruction.value, &instruction.span))
        .or_else(|| desugar_relative_jump(&instruction.value, &instruction.span))
        .or_else(|| desugar_call(&instruction.value, &instruction.span))
        .or_else(|| desugar_stack(&instruction.value, &instruction.span))
        .or_else(|| desugar_memory(&instruction.value, &instruction.span))
        .or_else(|| desugar_zero(&instruction.value, &instruction.span))
        .or_else(|| desugar_inc_dec(&instruction.value, &instruction.span))
        .or_else(|| desugar_not(&instruction.value, &instruction.span))
        .or_else(|| desugar_cmpi(&instruction.value, &instruction.span))
        .or_else(|| desugar_shift_immediate(&instruction.value, &instruction.span))
        .or_else(|| desugar_swap(&instruction.value, &instruction.span))
        .or_else(|| desugar_address_pseudo(&instruction.value, &instruction.span))
        .or_else(|| desugar_inc16(&instruction.value, &instruction.span, id))
        .or_else(|| desugar_mul(&instruction.value, &instruction.span, id))
        .or_else(|| desugar_ld_offset(&instruction.value, &instruction.span, id))
        .ok_or_else(|| AsmError {
            span: Some(instruction.span.clone()),
            kind: AsmErrorKind::UnsupportedInstruction {
                mnemonic: instruction.value.mnemonic.clone(),
                operands: instruction.value.operands.clone(),
            },
        })
}

fn desugar_system(instruction: &ASTInstruction, span: &Span) -> Option<Vec<IRItem>> {
    match (
        instruction.mnemonic.as_str(),
        instruction.operands.first(),
        instruction.operands.get(1),
    ) {
        ("nop", None, None) => Some(vec![IRItem::Instruction(Spanned {
            value: IRInstruction::Nop,
            span: span.clone(),
        })]),
        ("halt", None, None) => Some(vec![IRItem::Instruction(Spanned {
            value: IRInstruction::Halt { code: 0 },
            span: span.clone(),
        })]),
        ("halt", Some(Operand::Raw(DataSource::Immediate(code))), None) => {
            u8::try_from(*code).ok().map(|code| {
                vec![IRItem::Instruction(Spanned {
                    value: IRInstruction::Halt { code },
                    span: span.clone(),
                })]
            })
        }
        ("sys", None, None) => Some(vec![IRItem::Instruction(Spanned {
            value: IRInstruction::Sys,
            span: span.clone(),
        })]),
        _ => None,
    }
}

fn desugar_reg_reg(instruction: &ASTInstruction, span: &Span) -> Option<Vec<IRItem>> {
    let (
        Some(Operand::Raw(DataSource::Register(dst))),
        Some(Operand::Raw(DataSource::Register(src))),
        None,
    ) = (
        instruction.operands.first(),
        instruction.operands.get(1),
        instruction.operands.get(2),
    )
    else {
        return None;
    };

    let instruction = match instruction.mnemonic.as_str() {
        "mov" => IRInstruction::Mov {
            dst: *dst,
            src: *src,
        },
        "add" => IRInstruction::Add {
            dst: *dst,
            src: *src,
        },
        "sub" => IRInstruction::Sub {
            dst: *dst,
            src: *src,
        },
        "and" => IRInstruction::And {
            dst: *dst,
            src: *src,
        },
        "or" => IRInstruction::Or {
            dst: *dst,
            src: *src,
        },
        "xor" => IRInstruction::Xor {
            dst: *dst,
            src: *src,
        },
        "shr" => IRInstruction::Shr {
            dst: *dst,
            src: *src,
        },
        "shl" => IRInstruction::Shl {
            dst: *dst,
            src: *src,
        },
        "cmp" => IRInstruction::Cmp {
            dst: *dst,
            src: *src,
        },
        _ => return None,
    };
    Some(vec![IRItem::Instruction(Spanned {
        value: instruction,
        span: span.clone(),
    })])
}

fn desugar_ldi(instruction: &ASTInstruction, span: &Span) -> Option<Vec<IRItem>> {
    match (
        instruction.mnemonic.as_str(),
        instruction.operands.first(),
        instruction.operands.get(1),
        instruction.operands.get(2),
    ) {
        (
            "ldi",
            Some(Operand::Raw(DataSource::Register(dst))),
            Some(Operand::Raw(DataSource::Immediate(value))),
            None,
        ) => Some(vec![IRItem::Instruction(Spanned {
            value: IRInstruction::Ldi {
                dst: *dst,
                src: Expression::Immediate(*value),
            },
            span: span.clone(),
        })]),
        (
            "ldi",
            Some(Operand::Raw(DataSource::RegisterPair(hi, lo))),
            Some(Operand::Raw(DataSource::Immediate(value))),
            None,
        ) => Some(vec![
            IRItem::Instruction(Spanned {
                value: IRInstruction::Ldi {
                    dst: *hi,
                    src: Expression::Immediate(value >> 8),
                },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::Ldi {
                    dst: *lo,
                    src: Expression::Immediate(value & 0x00FF),
                },
                span: span.clone(),
            }),
        ]),
        (
            "ldi",
            Some(Operand::Raw(DataSource::RegisterPair(hi, lo))),
            Some(Operand::Raw(DataSource::Label(label))),
            None,
        ) => Some(vec![
            IRItem::Instruction(Spanned {
                value: IRInstruction::Ldi {
                    dst: *hi,
                    src: Expression::Hi(Box::new(Expression::Label(label.clone()))),
                },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::Ldi {
                    dst: *lo,
                    src: Expression::Lo(Box::new(Expression::Label(label.clone()))),
                },
                span: span.clone(),
            }),
        ]),
        _ => None,
    }
}

fn desugar_jmp(instruction: &ASTInstruction, span: &Span) -> Option<Vec<IRItem>> {
    match (
        instruction.mnemonic.as_str(),
        instruction.operands.first(),
        instruction.operands.get(1),
    ) {
        ("jmp", Some(Operand::MemoryWrapped(DataSource::RegisterPair(hi, lo))), None) => {
            Some(vec![IRItem::Instruction(Spanned {
                value: IRInstruction::Jmp { hi: *hi, lo: *lo },
                span: span.clone(),
            })])
        }
        ("jmp", Some(Operand::MemoryWrapped(DataSource::Label(name))), None) => Some(vec![
            IRItem::Instruction(Spanned {
                value: IRInstruction::Ldi {
                    dst: Register::IH,
                    src: Expression::Hi(Box::new(Expression::Label(name.clone()))),
                },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::Ldi {
                    dst: Register::IL,
                    src: Expression::Lo(Box::new(Expression::Label(name.clone()))),
                },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::Jmp {
                    hi: Register::IH,
                    lo: Register::IL,
                },
                span: span.clone(),
            }),
        ]),
        ("jmp", Some(Operand::MemoryWrapped(DataSource::Immediate(address))), None) => Some(vec![
            IRItem::Instruction(Spanned {
                value: IRInstruction::Ldi {
                    dst: Register::IH,
                    src: Expression::Immediate(address >> 8),
                },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::Ldi {
                    dst: Register::IL,
                    src: Expression::Immediate(address & 0x00FF),
                },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::Jmp {
                    hi: Register::IH,
                    lo: Register::IL,
                },
                span: span.clone(),
            }),
        ]),
        _ => None,
    }
}

fn desugar_relative_jump(instruction: &ASTInstruction, span: &Span) -> Option<Vec<IRItem>> {
    if instruction.operands.get(1).is_some() {
        return None;
    }

    let offset = match instruction.operands.first()? {
        Operand::Raw(DataSource::Immediate(offset)) => RelativeOffset::Immediate(*offset),
        Operand::MemoryWrapped(DataSource::Immediate(address)) => {
            RelativeOffset::Address(Expression::Immediate(*address))
        }
        Operand::MemoryWrapped(DataSource::Label(label)) => {
            RelativeOffset::Address(Expression::Label(label.clone()))
        }
        _ => return None,
    };
    let instruction = match instruction.mnemonic.as_str() {
        "jr" => IRInstruction::Jr { offset },
        "jzr" => IRInstruction::Jzr { offset },
        "jnzr" => IRInstruction::Jnzr { offset },
        "jcr" => IRInstruction::Jcr { offset },
        "jncr" => IRInstruction::Jncr { offset },
        _ => return None,
    };
    Some(vec![IRItem::Instruction(Spanned {
        value: instruction,
        span: span.clone(),
    })])
}

fn desugar_call(instruction: &ASTInstruction, span: &Span) -> Option<Vec<IRItem>> {
    match (
        instruction.mnemonic.as_str(),
        instruction.operands.first(),
        instruction.operands.get(1),
    ) {
        ("call", Some(Operand::MemoryWrapped(DataSource::RegisterPair(hi, lo))), None) => {
            Some(vec![IRItem::Instruction(Spanned {
                value: IRInstruction::Call { hi: *hi, lo: *lo },
                span: span.clone(),
            })])
        }
        ("call", Some(Operand::MemoryWrapped(DataSource::Immediate(address))), None) => Some(vec![
            IRItem::Instruction(Spanned {
                value: IRInstruction::Ldi {
                    dst: Register::IH,
                    src: Expression::Immediate(address >> 8),
                },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::Ldi {
                    dst: Register::IL,
                    src: Expression::Immediate(address & 0x00FF),
                },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::Call {
                    hi: Register::IH,
                    lo: Register::IL,
                },
                span: span.clone(),
            }),
        ]),
        ("call", Some(Operand::MemoryWrapped(DataSource::Label(label))), None) => Some(vec![
            IRItem::Instruction(Spanned {
                value: IRInstruction::Ldi {
                    dst: Register::IH,
                    src: Expression::Hi(Box::new(Expression::Label(label.clone()))),
                },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::Ldi {
                    dst: Register::IL,
                    src: Expression::Lo(Box::new(Expression::Label(label.clone()))),
                },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::Call {
                    hi: Register::IH,
                    lo: Register::IL,
                },
                span: span.clone(),
            }),
        ]),
        _ => None,
    }
}

fn desugar_stack(instruction: &ASTInstruction, span: &Span) -> Option<Vec<IRItem>> {
    match (
        instruction.mnemonic.as_str(),
        instruction.operands.first(),
        instruction.operands.get(1),
    ) {
        ("ret", None, None) => Some(vec![IRItem::Instruction(Spanned {
            value: IRInstruction::Ret,
            span: span.clone(),
        })]),
        ("push", Some(Operand::Raw(DataSource::Register(src))), None) => {
            Some(vec![IRItem::Instruction(Spanned {
                value: IRInstruction::Push { src: *src },
                span: span.clone(),
            })])
        }
        ("pop", Some(Operand::Raw(DataSource::Register(dst))), None) => {
            Some(vec![IRItem::Instruction(Spanned {
                value: IRInstruction::Pop { dst: *dst },
                span: span.clone(),
            })])
        }
        _ => None,
    }
}

fn desugar_memory(instruction: &ASTInstruction, span: &Span) -> Option<Vec<IRItem>> {
    match (
        instruction.mnemonic.as_str(),
        instruction.operands.first(),
        instruction.operands.get(1),
        instruction.operands.get(2),
    ) {
        (
            "ld",
            Some(Operand::Raw(DataSource::Register(dst))),
            Some(Operand::MemoryWrapped(DataSource::RegisterPair(hi, lo))),
            None,
        ) => Some(vec![IRItem::Instruction(Spanned {
            value: IRInstruction::Ld {
                dst: *dst,
                hi: *hi,
                lo: *lo,
            },
            span: span.clone(),
        })]),
        (
            "st",
            Some(Operand::MemoryWrapped(DataSource::RegisterPair(hi, lo))),
            Some(Operand::Raw(DataSource::Register(src))),
            None,
        ) => Some(vec![IRItem::Instruction(Spanned {
            value: IRInstruction::St {
                src: *src,
                hi: *hi,
                lo: *lo,
            },
            span: span.clone(),
        })]),
        _ => None,
    }
}

fn desugar_zero(instruction: &ASTInstruction, span: &Span) -> Option<Vec<IRItem>> {
    match (
        instruction.mnemonic.as_str(),
        instruction.operands.first(),
        instruction.operands.get(1),
    ) {
        ("zero", Some(Operand::Raw(DataSource::Register(reg))), None) => {
            Some(vec![IRItem::Instruction(Spanned {
                value: IRInstruction::Ldi {
                    dst: *reg,
                    src: Expression::Immediate(0),
                },
                span: span.clone(),
            })])
        }
        _ => None,
    }
}

fn desugar_inc_dec(instruction: &ASTInstruction, span: &Span) -> Option<Vec<IRItem>> {
    match (
        instruction.mnemonic.as_str(),
        instruction.operands.first(),
        instruction.operands.get(1),
    ) {
        ("inc", Some(Operand::Raw(DataSource::Register(reg))), None) => Some(vec![
            IRItem::Instruction(Spanned {
                value: IRInstruction::Push { src: Register::R0 },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::Ldi {
                    dst: Register::R0,
                    src: Expression::Immediate(1),
                },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::Add {
                    dst: *reg,
                    src: Register::R0,
                },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::Pop { dst: Register::R0 },
                span: span.clone(),
            }),
        ]),
        ("dec", Some(Operand::Raw(DataSource::Register(reg))), None) => Some(vec![
            IRItem::Instruction(Spanned {
                value: IRInstruction::Push { src: Register::R0 },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::Ldi {
                    dst: Register::R0,
                    src: Expression::Immediate(1),
                },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::Sub {
                    dst: *reg,
                    src: Register::R0,
                },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::Pop { dst: Register::R0 },
                span: span.clone(),
            }),
        ]),
        _ => None,
    }
}

fn desugar_not(instruction: &ASTInstruction, span: &Span) -> Option<Vec<IRItem>> {
    match (
        instruction.mnemonic.as_str(),
        instruction.operands.first(),
        instruction.operands.get(1),
    ) {
        ("not", Some(Operand::Raw(DataSource::Register(reg))), None) => Some(vec![
            IRItem::Instruction(Spanned {
                value: IRInstruction::Push { src: Register::R0 },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::Ldi {
                    dst: Register::R0,
                    src: Expression::Immediate(0xFF),
                },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::Xor {
                    dst: *reg,
                    src: Register::R0,
                },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::Pop { dst: Register::R0 },
                span: span.clone(),
            }),
        ]),
        _ => None,
    }
}

fn desugar_cmpi(instruction: &ASTInstruction, span: &Span) -> Option<Vec<IRItem>> {
    match (
        instruction.mnemonic.as_str(),
        instruction.operands.first(),
        instruction.operands.get(1),
        instruction.operands.get(2),
    ) {
        (
            "cmpi",
            Some(Operand::Raw(DataSource::Register(reg))),
            Some(Operand::Raw(DataSource::Immediate(value))),
            None,
        ) => Some(vec![
            IRItem::Instruction(Spanned {
                value: IRInstruction::Push { src: Register::R0 },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::Ldi {
                    dst: Register::R0,
                    src: Expression::Immediate(*value),
                },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::Sub {
                    dst: Register::R0,
                    src: *reg,
                },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::Pop { dst: Register::R0 },
                span: span.clone(),
            }),
        ]),
        _ => None,
    }
}

fn desugar_shift_immediate(instruction: &ASTInstruction, span: &Span) -> Option<Vec<IRItem>> {
    match (
        instruction.mnemonic.as_str(),
        instruction.operands.first(),
        instruction.operands.get(1),
        instruction.operands.get(2),
    ) {
        (
            "shri",
            Some(Operand::Raw(DataSource::Register(reg))),
            Some(Operand::Raw(DataSource::Immediate(value))),
            None,
        ) => Some(vec![
            IRItem::Instruction(Spanned {
                value: IRInstruction::Push { src: Register::R0 },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::Ldi {
                    dst: Register::R0,
                    src: Expression::Immediate(*value),
                },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::Shr {
                    dst: *reg,
                    src: Register::R0,
                },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::Pop { dst: Register::R0 },
                span: span.clone(),
            }),
        ]),
        (
            "shli",
            Some(Operand::Raw(DataSource::Register(reg))),
            Some(Operand::Raw(DataSource::Immediate(value))),
            None,
        ) => Some(vec![
            IRItem::Instruction(Spanned {
                value: IRInstruction::Push { src: Register::R0 },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::Ldi {
                    dst: Register::R0,
                    src: Expression::Immediate(*value),
                },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::Shl {
                    dst: *reg,
                    src: Register::R0,
                },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::Pop { dst: Register::R0 },
                span: span.clone(),
            }),
        ]),
        _ => None,
    }
}

fn desugar_swap(instruction: &ASTInstruction, span: &Span) -> Option<Vec<IRItem>> {
    match (
        instruction.mnemonic.as_str(),
        instruction.operands.first(),
        instruction.operands.get(1),
        instruction.operands.get(2),
    ) {
        (
            "swap",
            Some(Operand::Raw(DataSource::Register(left))),
            Some(Operand::Raw(DataSource::Register(right))),
            None,
        ) => Some(vec![
            IRItem::Instruction(Spanned {
                value: IRInstruction::Push { src: *left },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::Mov {
                    dst: *left,
                    src: *right,
                },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::Pop { dst: *right },
                span: span.clone(),
            }),
        ]),
        _ => None,
    }
}

#[allow(clippy::too_many_lines)]
fn desugar_address_pseudo(instruction: &ASTInstruction, span: &Span) -> Option<Vec<IRItem>> {
    match (
        instruction.mnemonic.as_str(),
        instruction.operands.first(),
        instruction.operands.get(1),
        instruction.operands.get(2),
    ) {
        (
            "ld",
            Some(Operand::Raw(DataSource::Register(dst))),
            Some(Operand::MemoryWrapped(DataSource::Immediate(address))),
            None,
        ) => Some(vec![
            IRItem::Instruction(Spanned {
                value: IRInstruction::Ldi {
                    dst: Register::IH,
                    src: Expression::Immediate(address >> 8),
                },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::Ldi {
                    dst: Register::IL,
                    src: Expression::Immediate(address & 0x00FF),
                },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::Ld {
                    dst: *dst,
                    hi: Register::IH,
                    lo: Register::IL,
                },
                span: span.clone(),
            }),
        ]),
        (
            "ld",
            Some(Operand::Raw(DataSource::Register(dst))),
            Some(Operand::MemoryWrapped(DataSource::Label(label))),
            None,
        ) => Some(vec![
            IRItem::Instruction(Spanned {
                value: IRInstruction::Ldi {
                    dst: Register::IH,
                    src: Expression::Hi(Box::new(Expression::Label(label.clone()))),
                },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::Ldi {
                    dst: Register::IL,
                    src: Expression::Lo(Box::new(Expression::Label(label.clone()))),
                },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::Ld {
                    dst: *dst,
                    hi: Register::IH,
                    lo: Register::IL,
                },
                span: span.clone(),
            }),
        ]),
        (
            "st",
            Some(Operand::MemoryWrapped(DataSource::Immediate(address))),
            Some(Operand::Raw(DataSource::Register(src))),
            None,
        ) => Some(vec![
            IRItem::Instruction(Spanned {
                value: IRInstruction::Ldi {
                    dst: Register::IH,
                    src: Expression::Immediate(address >> 8),
                },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::Ldi {
                    dst: Register::IL,
                    src: Expression::Immediate(address & 0x00FF),
                },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::St {
                    src: *src,
                    hi: Register::IH,
                    lo: Register::IL,
                },
                span: span.clone(),
            }),
        ]),
        (
            "st",
            Some(Operand::MemoryWrapped(DataSource::Label(label))),
            Some(Operand::Raw(DataSource::Register(src))),
            None,
        ) => Some(vec![
            IRItem::Instruction(Spanned {
                value: IRInstruction::Ldi {
                    dst: Register::IH,
                    src: Expression::Hi(Box::new(Expression::Label(label.clone()))),
                },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::Ldi {
                    dst: Register::IL,
                    src: Expression::Lo(Box::new(Expression::Label(label.clone()))),
                },
                span: span.clone(),
            }),
            IRItem::Instruction(Spanned {
                value: IRInstruction::St {
                    src: *src,
                    hi: Register::IH,
                    lo: Register::IL,
                },
                span: span.clone(),
            }),
        ]),
        _ => None,
    }
}

#[allow(clippy::too_many_lines)]
fn desugar_inc16(instruction: &ASTInstruction, span: &Span, id: usize) -> Option<Vec<IRItem>> {
    match (
        instruction.mnemonic.as_str(),
        instruction.operands.first(),
        instruction.operands.get(1),
        instruction.operands.get(2),
    ) {
        (
            "inc16",
            Some(Operand::Raw(DataSource::Register(hi))),
            Some(Operand::Raw(DataSource::Register(lo))),
            None,
        ) => {
            let inc_hi = format!("__mb8_inc16_hi_{id}");
            let end = format!("__mb8_inc16_end_{id}");
            let mut items = vec![
                IRItem::Instruction(Spanned {
                    value: IRInstruction::Push { src: Register::R0 },
                    span: span.clone(),
                }),
                IRItem::Instruction(Spanned {
                    value: IRInstruction::Ldi {
                        dst: Register::R0,
                        src: Expression::Immediate(0xFF),
                    },
                    span: span.clone(),
                }),
                IRItem::Instruction(Spanned {
                    value: IRInstruction::Sub {
                        dst: Register::R0,
                        src: *lo,
                    },
                    span: span.clone(),
                }),
                IRItem::Instruction(Spanned {
                    value: IRInstruction::Pop { dst: Register::R0 },
                    span: span.clone(),
                }),
            ];
            items.push(IRItem::Instruction(Spanned {
                value: IRInstruction::Jzr {
                    offset: RelativeOffset::Address(Expression::Label(Spanned {
                        value: inc_hi.clone(),
                        span: span.clone(),
                    })),
                },
                span: span.clone(),
            }));
            items.extend([
                IRItem::Instruction(Spanned {
                    value: IRInstruction::Push { src: Register::R0 },
                    span: span.clone(),
                }),
                IRItem::Instruction(Spanned {
                    value: IRInstruction::Ldi {
                        dst: Register::R0,
                        src: Expression::Immediate(1),
                    },
                    span: span.clone(),
                }),
                IRItem::Instruction(Spanned {
                    value: IRInstruction::Add {
                        dst: *lo,
                        src: Register::R0,
                    },
                    span: span.clone(),
                }),
                IRItem::Instruction(Spanned {
                    value: IRInstruction::Pop { dst: Register::R0 },
                    span: span.clone(),
                }),
            ]);
            items.push(IRItem::Instruction(Spanned {
                value: IRInstruction::Jr {
                    offset: RelativeOffset::Address(Expression::Label(Spanned {
                        value: end.clone(),
                        span: span.clone(),
                    })),
                },
                span: span.clone(),
            }));
            items.push(IRItem::Label(Spanned {
                value: inc_hi,
                span: span.clone(),
            }));
            items.push(IRItem::Instruction(Spanned {
                value: IRInstruction::Ldi {
                    dst: *lo,
                    src: Expression::Immediate(0),
                },
                span: span.clone(),
            }));
            items.extend([
                IRItem::Instruction(Spanned {
                    value: IRInstruction::Push { src: Register::R0 },
                    span: span.clone(),
                }),
                IRItem::Instruction(Spanned {
                    value: IRInstruction::Ldi {
                        dst: Register::R0,
                        src: Expression::Immediate(1),
                    },
                    span: span.clone(),
                }),
                IRItem::Instruction(Spanned {
                    value: IRInstruction::Add {
                        dst: *hi,
                        src: Register::R0,
                    },
                    span: span.clone(),
                }),
                IRItem::Instruction(Spanned {
                    value: IRInstruction::Pop { dst: Register::R0 },
                    span: span.clone(),
                }),
            ]);
            items.push(IRItem::Label(Spanned {
                value: end,
                span: span.clone(),
            }));
            items.push(IRItem::Instruction(Spanned {
                value: IRInstruction::Nop,
                span: span.clone(),
            }));
            Some(items)
        }
        _ => None,
    }
}

#[allow(clippy::too_many_lines)]
fn desugar_mul(instruction: &ASTInstruction, span: &Span, id: usize) -> Option<Vec<IRItem>> {
    match (
        instruction.mnemonic.as_str(),
        instruction.operands.first(),
        instruction.operands.get(1),
        instruction.operands.get(2),
        instruction.operands.get(3),
    ) {
        (
            "mul",
            Some(Operand::Raw(DataSource::Register(dst))),
            Some(Operand::Raw(DataSource::Register(left))),
            Some(Operand::Raw(DataSource::Register(right))),
            None,
        ) => {
            let iter = format!("__mb8_mul_iter_{id}");
            let mut items = vec![
                IRItem::Instruction(Spanned {
                    value: IRInstruction::Ldi {
                        dst: *dst,
                        src: Expression::Immediate(0),
                    },
                    span: span.clone(),
                }),
                IRItem::Instruction(Spanned {
                    value: IRInstruction::Push { src: *right },
                    span: span.clone(),
                }),
                IRItem::Label(Spanned {
                    value: iter.clone(),
                    span: span.clone(),
                }),
                IRItem::Instruction(Spanned {
                    value: IRInstruction::Add {
                        dst: *dst,
                        src: *left,
                    },
                    span: span.clone(),
                }),
            ];
            items.extend([
                IRItem::Instruction(Spanned {
                    value: IRInstruction::Push { src: Register::R0 },
                    span: span.clone(),
                }),
                IRItem::Instruction(Spanned {
                    value: IRInstruction::Ldi {
                        dst: Register::R0,
                        src: Expression::Immediate(1),
                    },
                    span: span.clone(),
                }),
                IRItem::Instruction(Spanned {
                    value: IRInstruction::Sub {
                        dst: *right,
                        src: Register::R0,
                    },
                    span: span.clone(),
                }),
                IRItem::Instruction(Spanned {
                    value: IRInstruction::Pop { dst: Register::R0 },
                    span: span.clone(),
                }),
                IRItem::Instruction(Spanned {
                    value: IRInstruction::Push { src: Register::R0 },
                    span: span.clone(),
                }),
                IRItem::Instruction(Spanned {
                    value: IRInstruction::Ldi {
                        dst: Register::R0,
                        src: Expression::Immediate(0),
                    },
                    span: span.clone(),
                }),
                IRItem::Instruction(Spanned {
                    value: IRInstruction::Sub {
                        dst: Register::R0,
                        src: *right,
                    },
                    span: span.clone(),
                }),
                IRItem::Instruction(Spanned {
                    value: IRInstruction::Pop { dst: Register::R0 },
                    span: span.clone(),
                }),
            ]);
            items.push(IRItem::Instruction(Spanned {
                value: IRInstruction::Jnzr {
                    offset: RelativeOffset::Address(Expression::Label(Spanned {
                        value: iter,
                        span: span.clone(),
                    })),
                },
                span: span.clone(),
            }));
            items.push(IRItem::Instruction(Spanned {
                value: IRInstruction::Pop { dst: *right },
                span: span.clone(),
            }));
            Some(items)
        }
        _ => None,
    }
}

fn desugar_ld_offset(instruction: &ASTInstruction, span: &Span, id: usize) -> Option<Vec<IRItem>> {
    match (
        instruction.mnemonic.as_str(),
        instruction.operands.first(),
        instruction.operands.get(1),
        instruction.operands.get(2),
    ) {
        (
            "ld",
            Some(Operand::Raw(DataSource::Register(dst))),
            Some(Operand::MemoryOffset { hi, lo, offset }),
            None,
        ) => {
            let no_borrow = format!("__mb8_ld_no_borrow_{id}");
            let mut items = vec![
                IRItem::Instruction(Spanned {
                    value: IRInstruction::Ldi {
                        dst: Register::R0,
                        src: Expression::Immediate(u16::from(*offset)),
                    },
                    span: span.clone(),
                }),
                IRItem::Instruction(Spanned {
                    value: IRInstruction::Sub {
                        dst: *lo,
                        src: Register::R0,
                    },
                    span: span.clone(),
                }),
                IRItem::Instruction(Spanned {
                    value: IRInstruction::Jncr {
                        offset: RelativeOffset::Address(Expression::Label(Spanned {
                            value: no_borrow.clone(),
                            span: span.clone(),
                        })),
                    },
                    span: span.clone(),
                }),
            ];
            items.extend([
                IRItem::Instruction(Spanned {
                    value: IRInstruction::Push { src: Register::R7 },
                    span: span.clone(),
                }),
                IRItem::Instruction(Spanned {
                    value: IRInstruction::Ldi {
                        dst: Register::R7,
                        src: Expression::Immediate(1),
                    },
                    span: span.clone(),
                }),
                IRItem::Instruction(Spanned {
                    value: IRInstruction::Sub {
                        dst: *hi,
                        src: Register::R7,
                    },
                    span: span.clone(),
                }),
                IRItem::Instruction(Spanned {
                    value: IRInstruction::Pop { dst: Register::R7 },
                    span: span.clone(),
                }),
            ]);
            items.push(IRItem::Label(Spanned {
                value: no_borrow,
                span: span.clone(),
            }));
            items.push(IRItem::Instruction(Spanned {
                value: IRInstruction::Ld {
                    dst: *dst,
                    hi: *hi,
                    lo: *lo,
                },
                span: span.clone(),
            }));
            Some(items)
        }
        _ => None,
    }
}

pub fn desugar(ast: &ASTProgram) -> Result<IRProgram, AsmError> {
    let mut result = IRProgram {
        origin: None,
        items: Vec::new(),
    };
    for (id, item) in ast.items.iter().enumerate() {
        match item {
            ASTItem::Instruction(inst) => {
                result.items.extend(desugar_instruction(inst, id)?);
            }
            ASTItem::Label(label) => {
                result.items.push(IRItem::Label(label.clone()));
            }
            ASTItem::Directive(directive) => match &directive.value {
                Directive::Origin(address) => {
                    if let Some(origin) = &result.origin {
                        return Err(AsmError {
                            span: Some(directive.span.clone()),
                            kind: AsmErrorKind::DuplicateOrigin {
                                first: origin.span.clone(),
                            },
                        });
                    }
                    result.origin = Some(Spanned {
                        value: *address,
                        span: directive.span.clone(),
                    });
                }
                Directive::Data(bytes) => result.items.push(IRItem::Data(Spanned {
                    value: bytes.clone(),
                    span: directive.span.clone(),
                })),
                Directive::Ascii(text) => result.items.push(IRItem::Data(Spanned {
                    value: text.as_bytes().to_vec(),
                    span: directive.span.clone(),
                })),
                Directive::Include(_) => {
                    return Err(AsmError {
                        span: Some(directive.span.clone()),
                        kind: AsmErrorKind::UnexpectedDirective {
                            directive: directive.value.clone(),
                        },
                    });
                }
            },
        }
    }
    Ok(result)
}
