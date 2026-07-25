use mb8_isa::registers::Register;

use crate::{
    ast::{ASTInstruction, DataSource, Operand},
    diagnostics::{Span, Spanned},
    ir::{Expression, IRInstruction, IRItem},
};

use super::super::{InstructionDefinition, RegisterEffect, RegisterSet};

pub(in crate::instructions) const DESUGAR: InstructionDefinition = InstructionDefinition {
    mnemonic: "inc",
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
    let ("inc", Some(Operand::Raw(DataSource::Register(reg))), None) = (
        instruction.mnemonic.as_str(),
        instruction.operands.first(),
        instruction.operands.get(1),
    ) else {
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
                src: Expression::Immediate(1),
            },
            span: span.clone(),
        }),
        IRItem::Instruction(Spanned {
            value: IRInstruction::Add {
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
