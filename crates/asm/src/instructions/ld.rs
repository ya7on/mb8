use crate::{
    ast::{ASTInstruction, DataSource, Operand},
    diagnostics::{Span, Spanned},
    ir::{IRInstruction, IRItem},
};

use super::InstructionDefinition;

pub(super) const DESUGAR: InstructionDefinition = InstructionDefinition {
    mnemonic: "ld",
    handler: desugar,
};

pub(super) fn desugar(
    instruction: &ASTInstruction,
    span: &Span,
    _id: usize,
) -> Option<Vec<IRItem>> {
    let (
        "ld",
        Some(Operand::Raw(DataSource::Register(dst))),
        Some(Operand::MemoryWrapped(DataSource::RegisterPair(hi, lo))),
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
        value: IRInstruction::Ld {
            dst: *dst,
            hi: *hi,
            lo: *lo,
        },
        span: span.clone(),
    })])
}
