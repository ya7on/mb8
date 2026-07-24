use crate::{
    ast::{ASTInstruction, DataSource, Operand},
    diagnostics::{Span, Spanned},
    ir::{IRInstruction, IRItem, RelativeOffset},
};

use super::InstructionDefinition;

pub(super) const DESUGAR: InstructionDefinition = InstructionDefinition {
    mnemonic: "jzr",
    handler: desugar,
};

pub(super) fn desugar(
    instruction: &ASTInstruction,
    span: &Span,
    _id: usize,
) -> Option<Vec<IRItem>> {
    let ("jzr", Some(Operand::Raw(DataSource::Immediate(offset))), None) = (
        instruction.mnemonic.as_str(),
        instruction.operands.first(),
        instruction.operands.get(1),
    ) else {
        return None;
    };

    Some(vec![IRItem::Instruction(Spanned {
        value: IRInstruction::Jzr {
            offset: RelativeOffset::Immediate(*offset),
        },
        span: span.clone(),
    })])
}
