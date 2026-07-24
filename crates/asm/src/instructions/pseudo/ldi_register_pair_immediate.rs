use crate::{
    ast::{ASTInstruction, DataSource, Operand},
    diagnostics::{Span, Spanned},
    ir::{Expression, IRInstruction, IRItem},
};

use super::super::{InstructionDefinition, RegisterEffect, RegisterSet};

pub(in crate::instructions) const DESUGAR: InstructionDefinition = InstructionDefinition {
    mnemonic: "ldi",
    handler: desugar,
    effect: RegisterEffect {
        scratch: RegisterSet::EMPTY,
    },
};

pub(super) fn desugar(
    instruction: &ASTInstruction,
    span: &Span,
    _id: usize,
) -> Option<Vec<IRItem>> {
    let (
        "ldi",
        Some(Operand::Raw(DataSource::RegisterPair(hi, lo))),
        Some(Operand::Raw(DataSource::Immediate(value))),
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

    Some(vec![
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
    ])
}
