use mb8_isa::registers::Register;

use crate::{
    ast::{ASTInstruction, DataSource, Operand},
    diagnostics::{Span, Spanned},
    ir::{Expression, IRInstruction, IRItem},
};

use super::super::{InstructionDefinition, RegisterEffect, RegisterSet};

pub(in crate::instructions) const DESUGAR: InstructionDefinition = InstructionDefinition {
    mnemonic: "call",
    handler: desugar,
    effect: RegisterEffect {
        scratch: RegisterSet::from_registers(&[Register::IH, Register::IL]),
    },
};

pub(super) fn desugar(
    instruction: &ASTInstruction,
    span: &Span,
    _id: usize,
) -> Option<Vec<IRItem>> {
    let ("call", Some(Operand::MemoryWrapped(DataSource::Immediate(address))), None) = (
        instruction.mnemonic.as_str(),
        instruction.operands.first(),
        instruction.operands.get(1),
    ) else {
        return None;
    };

    Some(vec![
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
    ])
}
