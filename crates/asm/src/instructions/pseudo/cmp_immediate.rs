use mb8_isa::registers::Register;

use crate::{
    ast::{ASTInstruction, DataSource, Operand},
    diagnostics::{Span, Spanned},
    instructions::{RegisterEffect, RegisterSet},
    ir::{Expression, IRInstruction, IRItem},
};

use super::super::InstructionDefinition;

pub(in crate::instructions) const DESUGAR: InstructionDefinition = InstructionDefinition {
    mnemonic: "cmp",
    handler: desugar,
    effect: RegisterEffect {
        scratch: RegisterSet::from_registers(&[Register::A]),
    },
};

pub(super) fn desugar(
    instruction: &ASTInstruction,
    span: &Span,
    _id: usize,
) -> Option<Vec<IRItem>> {
    let (
        "cmp",
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
            value: IRInstruction::Push { src: Register::A },
            span: span.clone(),
        }),
        IRItem::Instruction(Spanned {
            value: IRInstruction::Ldi {
                dst: Register::A,
                src: Expression::Immediate(*value),
            },
            span: span.clone(),
        }),
        IRItem::Instruction(Spanned {
            value: IRInstruction::Cmp {
                dst: *reg,
                src: Register::A,
            },
            span: span.clone(),
        }),
        IRItem::Instruction(Spanned {
            value: IRInstruction::Pop { dst: Register::A },
            span: span.clone(),
        }),
    ])
}
