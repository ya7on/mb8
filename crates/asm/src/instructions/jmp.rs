use crate::{
    ast::{ASTInstruction, DataSource, Operand},
    diagnostics::{Span, Spanned},
    ir::{IRInstruction, IRItem},
};

use super::{InstructionDefinition, RegisterEffect, RegisterSet};

pub(super) const DESUGAR: InstructionDefinition = InstructionDefinition {
    mnemonic: "jmp",
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
    let ("jmp", Some(Operand::MemoryWrapped(DataSource::RegisterPair(hi, lo))), None) = (
        instruction.mnemonic.as_str(),
        instruction.operands.first(),
        instruction.operands.get(1),
    ) else {
        return None;
    };

    Some(vec![IRItem::Instruction(Spanned {
        value: IRInstruction::Jmp { hi: *hi, lo: *lo },
        span: span.clone(),
    })])
}
