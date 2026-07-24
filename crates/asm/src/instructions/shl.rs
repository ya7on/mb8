use crate::{
    ast::{ASTInstruction, DataSource, Operand},
    diagnostics::{Span, Spanned},
    ir::{IRInstruction, IRItem},
};

use super::{InstructionDefinition, RegisterEffect, RegisterSet};

pub(super) const DESUGAR: InstructionDefinition = InstructionDefinition {
    mnemonic: "shl",
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
        "shl",
        Some(Operand::Raw(DataSource::Register(dst))),
        Some(Operand::Raw(DataSource::Register(src))),
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

    Some(vec![IRItem::Instruction(Spanned {
        value: IRInstruction::Shl {
            dst: *dst,
            src: *src,
        },
        span: span.clone(),
    })])
}
