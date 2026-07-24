use mb8_isa::registers::Register;

use crate::{
    ast::{ASTInstruction, DataSource, Operand},
    diagnostics::{Span, Spanned},
    ir::{Expression, IRInstruction, IRItem},
};

use super::super::InstructionDefinition;

pub(in crate::instructions) const DESUGAR: InstructionDefinition = InstructionDefinition {
    mnemonic: "st",
    handler: desugar,
};

pub(super) fn desugar(
    instruction: &ASTInstruction,
    span: &Span,
    _id: usize,
) -> Option<Vec<IRItem>> {
    let (
        "st",
        Some(Operand::MemoryWrapped(DataSource::Label(label))),
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

    Some(vec![
        IRItem::Instruction(Spanned {
            value: IRInstruction::Ldi {
                dst: Register::IH,
                src: Expression::Hi(Box::new(Expression::Label(label.clone()))),
            },
            span: span.clone(),
        }),
        IRItem::Instruction(Spanned {
            value: IRInstruction::Ldi {
                dst: Register::IL,
                src: Expression::Lo(Box::new(Expression::Label(label.clone()))),
            },
            span: span.clone(),
        }),
        IRItem::Instruction(Spanned {
            value: IRInstruction::St {
                src: *src,
                hi: Register::IH,
                lo: Register::IL,
            },
            span: span.clone(),
        }),
    ])
}
