use mb8_isa::registers::Register;

use crate::{
    ast::{ASTInstruction, DataSource, Operand},
    diagnostics::{Span, Spanned},
    ir::{Expression, IRInstruction, IRItem},
};

use super::{
    super::{InstructionDefinition, RegisterEffect, RegisterSet},
    memcpy::{inc16, instruction, label, relative_label},
};

pub(in crate::instructions) const DESUGAR: InstructionDefinition = InstructionDefinition {
    mnemonic: "strcmp",
    handler: desugar,
    effect: RegisterEffect {
        scratch: RegisterSet::EMPTY,
    },
};

fn absolute_jump(target: String, span: &Span) -> [IRItem; 3] {
    let expression = Expression::Label(Spanned {
        value: target,
        span: span.clone(),
    });
    [
        instruction(
            IRInstruction::Ldi {
                dst: Register::IH,
                src: Expression::Hi(Box::new(expression.clone())),
            },
            span,
        ),
        instruction(
            IRInstruction::Ldi {
                dst: Register::IL,
                src: Expression::Lo(Box::new(expression)),
            },
            span,
        ),
        instruction(
            IRInstruction::Jmp {
                hi: Register::IH,
                lo: Register::IL,
            },
            span,
        ),
    ]
}

#[allow(clippy::too_many_lines)]
pub(super) fn desugar(
    instruction_: &ASTInstruction,
    span: &Span,
    id: usize,
) -> Option<Vec<IRItem>> {
    let (
        "strcmp",
        Some(Operand::Raw(DataSource::Register(left))),
        Some(Operand::Raw(DataSource::Register(right))),
        Some(Operand::Raw(DataSource::Register(src_hi))),
        Some(Operand::Raw(DataSource::Register(src_lo))),
        Some(Operand::Raw(DataSource::Register(dst_hi))),
        Some(Operand::Raw(DataSource::Register(dst_lo))),
        None,
    ) = (
        instruction_.mnemonic.as_str(),
        instruction_.operands.first(),
        instruction_.operands.get(1),
        instruction_.operands.get(2),
        instruction_.operands.get(3),
        instruction_.operands.get(4),
        instruction_.operands.get(5),
        instruction_.operands.get(6),
    )
    else {
        return None;
    };

    let loop_label = format!("__mb8_strcmp_loop_{id}");
    let error_label = format!("__mb8_strcmp_error_{id}");
    let success_label = format!("__mb8_strcmp_success_{id}");
    let end_label = format!("__mb8_strcmp_end_{id}");
    let mut items = vec![
        label(loop_label.clone(), span),
        instruction(
            IRInstruction::Ld {
                dst: *left,
                hi: *src_hi,
                lo: *src_lo,
            },
            span,
        ),
        instruction(
            IRInstruction::Ld {
                dst: *right,
                hi: *dst_hi,
                lo: *dst_lo,
            },
            span,
        ),
        instruction(
            IRInstruction::Cmp {
                dst: *left,
                src: *right,
            },
            span,
        ),
        instruction(
            IRInstruction::Jnzr {
                offset: relative_label(error_label.clone(), span),
            },
            span,
        ),
        instruction(IRInstruction::Push { src: Register::A }, span),
        instruction(
            IRInstruction::Ldi {
                dst: Register::A,
                src: Expression::Immediate(0),
            },
            span,
        ),
        instruction(
            IRInstruction::Cmp {
                dst: *right,
                src: Register::A,
            },
            span,
        ),
        instruction(IRInstruction::Pop { dst: Register::A }, span),
        instruction(
            IRInstruction::Jzr {
                offset: relative_label(success_label.clone(), span),
            },
            span,
        ),
    ];
    items.extend(inc16(
        *src_hi,
        *src_lo,
        span,
        &format!("__mb8_strcmp_src_{id}"),
    ));
    items.extend(inc16(
        *dst_hi,
        *dst_lo,
        span,
        &format!("__mb8_strcmp_dst_{id}"),
    ));
    items.extend(absolute_jump(loop_label, span));
    items.extend([
        label(error_label, span),
        instruction(
            IRInstruction::Ldi {
                dst: *left,
                src: Expression::Immediate(1),
            },
            span,
        ),
        instruction(
            IRInstruction::Jr {
                offset: relative_label(end_label.clone(), span),
            },
            span,
        ),
        label(success_label, span),
        instruction(
            IRInstruction::Ldi {
                dst: *left,
                src: Expression::Immediate(0),
            },
            span,
        ),
        label(end_label, span),
    ]);

    Some(items)
}
