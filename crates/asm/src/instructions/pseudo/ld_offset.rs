use mb8_isa::registers::Register;

use crate::{
    ast::{ASTInstruction, DataSource, Operand},
    diagnostics::{Span, Spanned},
    ir::{Expression, IRInstruction, IRItem, RelativeOffset},
};

use super::super::{InstructionDefinition, RegisterEffect, RegisterSet};

pub(in crate::instructions) const DESUGAR: InstructionDefinition = InstructionDefinition {
    mnemonic: "ld",
    handler: desugar,
    effect: RegisterEffect {
        scratch: RegisterSet::from_registers(&[Register::A]),
    },
};

pub(super) fn desugar(instruction: &ASTInstruction, span: &Span, id: usize) -> Option<Vec<IRItem>> {
    let (
        "ld",
        Some(Operand::Raw(DataSource::Register(dst))),
        Some(Operand::MemoryOffset { hi, lo, offset }),
        None,
    ) = (
        instruction.mnemonic.as_str(),
        instruction.operands.first(),
        instruction.operands.get(1),
        instruction.operands.get(2),
    )
    else {
        return None;
    };

    let no_borrow = format!("__mb8_ld_no_borrow_{id}");
    let mut items = vec![
        IRItem::Instruction(Spanned {
            value: IRInstruction::Ldi {
                dst: Register::A,
                src: Expression::Immediate(u16::from(*offset)),
            },
            span: span.clone(),
        }),
        IRItem::Instruction(Spanned {
            value: IRInstruction::Sub {
                dst: *lo,
                src: Register::A,
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
            value: IRInstruction::Push { src: Register::A },
            span: span.clone(),
        }),
        IRItem::Instruction(Spanned {
            value: IRInstruction::Ldi {
                dst: Register::A,
                src: Expression::Immediate(1),
            },
            span: span.clone(),
        }),
        IRItem::Instruction(Spanned {
            value: IRInstruction::Sub {
                dst: *hi,
                src: Register::A,
            },
            span: span.clone(),
        }),
        IRItem::Instruction(Spanned {
            value: IRInstruction::Pop { dst: Register::A },
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
