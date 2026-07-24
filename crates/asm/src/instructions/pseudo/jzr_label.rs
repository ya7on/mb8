use crate::{
    ast::{ASTInstruction, DataSource, Operand},
    diagnostics::{Span, Spanned},
    ir::{Expression, IRInstruction, IRItem, RelativeOffset},
};

use super::super::InstructionDefinition;

pub(in crate::instructions) const DESUGAR: InstructionDefinition = InstructionDefinition {
    mnemonic: "jzr",
    handler: desugar,
};

pub(super) fn desugar(
    instruction: &ASTInstruction,
    span: &Span,
    _id: usize,
) -> Option<Vec<IRItem>> {
    let ("jzr", Some(Operand::MemoryWrapped(DataSource::Label(label))), None) = (
        instruction.mnemonic.as_str(),
        instruction.operands.first(),
        instruction.operands.get(1),
    ) else {
        return None;
    };

    Some(vec![IRItem::Instruction(Spanned {
        value: IRInstruction::Jzr {
            offset: RelativeOffset::Address(Expression::Label(label.clone())),
        },
        span: span.clone(),
    })])
}
