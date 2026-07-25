use mb8_isa::registers::Register;

use crate::{
    ast::{ASTInstruction, DataSource, Operand},
    diagnostics::{Span, Spanned},
    ir::{Expression, IRInstruction, IRItem, RelativeOffset},
};

use super::super::{InstructionDefinition, RegisterEffect, RegisterSet};

pub(in crate::instructions) const DESUGAR: InstructionDefinition = InstructionDefinition {
    mnemonic: "inc16",
    handler: desugar,
    effect: RegisterEffect {
        scratch: RegisterSet::from_registers(&[Register::R0]),
    },
};

#[allow(clippy::too_many_lines)]
pub(super) fn desugar(instruction: &ASTInstruction, span: &Span, id: usize) -> Option<Vec<IRItem>> {
    let ("inc16", Some(Operand::Raw(DataSource::RegisterPair(hi, lo))), None) = (
        instruction.mnemonic.as_str(),
        instruction.operands.first(),
        instruction.operands.get(1),
    ) else {
        return None;
    };

    let inc_hi = format!("__mb8_inc16_hi_{id}");
    let end = format!("__mb8_inc16_end_{id}");

    Some(vec![
        IRItem::Instruction(Spanned {
            value: IRInstruction::Push { src: Register::R0 },
            span: span.clone(),
        }),
        IRItem::Instruction(Spanned {
            value: IRInstruction::Ldi {
                dst: Register::R0,
                src: Expression::Immediate(0xFF),
            },
            span: span.clone(),
        }),
        IRItem::Instruction(Spanned {
            value: IRInstruction::Sub {
                dst: Register::R0,
                src: *lo,
            },
            span: span.clone(),
        }),
        IRItem::Instruction(Spanned {
            value: IRInstruction::Pop { dst: Register::R0 },
            span: span.clone(),
        }),
        IRItem::Instruction(Spanned {
            value: IRInstruction::Jzr {
                offset: RelativeOffset::Address(Expression::Label(Spanned {
                    value: inc_hi.clone(),
                    span: span.clone(),
                })),
            },
            span: span.clone(),
        }),
        IRItem::Instruction(Spanned {
            value: IRInstruction::Push { src: Register::R0 },
            span: span.clone(),
        }),
        IRItem::Instruction(Spanned {
            value: IRInstruction::Ldi {
                dst: Register::R0,
                src: Expression::Immediate(1),
            },
            span: span.clone(),
        }),
        IRItem::Instruction(Spanned {
            value: IRInstruction::Add {
                dst: *lo,
                src: Register::R0,
            },
            span: span.clone(),
        }),
        IRItem::Instruction(Spanned {
            value: IRInstruction::Pop { dst: Register::R0 },
            span: span.clone(),
        }),
        IRItem::Instruction(Spanned {
            value: IRInstruction::Jr {
                offset: RelativeOffset::Address(Expression::Label(Spanned {
                    value: end.clone(),
                    span: span.clone(),
                })),
            },
            span: span.clone(),
        }),
        IRItem::Label(Spanned {
            value: inc_hi,
            span: span.clone(),
        }),
        IRItem::Instruction(Spanned {
            value: IRInstruction::Ldi {
                dst: *lo,
                src: Expression::Immediate(0),
            },
            span: span.clone(),
        }),
        IRItem::Instruction(Spanned {
            value: IRInstruction::Push { src: Register::R0 },
            span: span.clone(),
        }),
        IRItem::Instruction(Spanned {
            value: IRInstruction::Ldi {
                dst: Register::R0,
                src: Expression::Immediate(1),
            },
            span: span.clone(),
        }),
        IRItem::Instruction(Spanned {
            value: IRInstruction::Add {
                dst: *hi,
                src: Register::R0,
            },
            span: span.clone(),
        }),
        IRItem::Instruction(Spanned {
            value: IRInstruction::Pop { dst: Register::R0 },
            span: span.clone(),
        }),
        IRItem::Label(Spanned {
            value: end,
            span: span.clone(),
        }),
        IRItem::Instruction(Spanned {
            value: IRInstruction::Nop,
            span: span.clone(),
        }),
    ])
}
