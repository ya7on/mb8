use mb8_isa::registers::Register;

use crate::{
    ast::{ASTInstruction, DataSource, Operand},
    diagnostics::{Span, Spanned},
    ir::{Expression, IRInstruction, IRItem, RelativeOffset},
};

use super::super::InstructionDefinition;

pub(in crate::instructions) const DESUGAR: InstructionDefinition = InstructionDefinition {
    mnemonic: "mul",
    handler: desugar,
};

#[allow(clippy::too_many_lines)]
pub(super) fn desugar(instruction: &ASTInstruction, span: &Span, id: usize) -> Option<Vec<IRItem>> {
    let (
        "mul",
        Some(Operand::Raw(DataSource::Register(dst))),
        Some(Operand::Raw(DataSource::Register(left))),
        Some(Operand::Raw(DataSource::Register(right))),
        None,
    ) = (
        instruction.mnemonic.as_str(),
        instruction.operands.first(),
        instruction.operands.get(1),
        instruction.operands.get(2),
        instruction.operands.get(3),
    )
    else {
        return None;
    };

    let iter = format!("__mb8_mul_iter_{id}");
    let mut items = vec![
        IRItem::Instruction(Spanned {
            value: IRInstruction::Ldi {
                dst: *dst,
                src: Expression::Immediate(0),
            },
            span: span.clone(),
        }),
        IRItem::Instruction(Spanned {
            value: IRInstruction::Push { src: *right },
            span: span.clone(),
        }),
        IRItem::Label(Spanned {
            value: iter.clone(),
            span: span.clone(),
        }),
        IRItem::Instruction(Spanned {
            value: IRInstruction::Add {
                dst: *dst,
                src: *left,
            },
            span: span.clone(),
        }),
    ];
    items.extend([
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
            value: IRInstruction::Sub {
                dst: *right,
                src: Register::R0,
            },
            span: span.clone(),
        }),
        IRItem::Instruction(Spanned {
            value: IRInstruction::Pop { dst: Register::R0 },
            span: span.clone(),
        }),
        IRItem::Instruction(Spanned {
            value: IRInstruction::Push { src: Register::R0 },
            span: span.clone(),
        }),
        IRItem::Instruction(Spanned {
            value: IRInstruction::Ldi {
                dst: Register::R0,
                src: Expression::Immediate(0),
            },
            span: span.clone(),
        }),
        IRItem::Instruction(Spanned {
            value: IRInstruction::Sub {
                dst: Register::R0,
                src: *right,
            },
            span: span.clone(),
        }),
        IRItem::Instruction(Spanned {
            value: IRInstruction::Pop { dst: Register::R0 },
            span: span.clone(),
        }),
    ]);
    items.push(IRItem::Instruction(Spanned {
        value: IRInstruction::Jnzr {
            offset: RelativeOffset::Address(Expression::Label(Spanned {
                value: iter,
                span: span.clone(),
            })),
        },
        span: span.clone(),
    }));
    items.push(IRItem::Instruction(Spanned {
        value: IRInstruction::Pop { dst: *right },
        span: span.clone(),
    }));
    Some(items)
}
