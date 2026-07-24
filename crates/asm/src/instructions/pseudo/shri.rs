use mb8_isa::registers::Register;

use crate::{
    ast::{ASTInstruction, DataSource, Operand},
    diagnostics::{Span, Spanned},
    ir::{Expression, IRInstruction, IRItem},
};

use super::super::InstructionDefinition;

pub(in crate::instructions) const DESUGAR: InstructionDefinition = InstructionDefinition {
    mnemonic: "shri",
    handler: desugar,
};

pub(super) fn desugar(
    instruction: &ASTInstruction,
    span: &Span,
    _id: usize,
) -> Option<Vec<IRItem>> {
    let (
        "shri",
        Some(Operand::Raw(DataSource::Register(reg))),
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
    ])
}
