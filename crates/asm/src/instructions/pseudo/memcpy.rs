use mb8_isa::registers::Register;

use crate::{
    ast::{ASTInstruction, DataSource, Operand},
    diagnostics::{Span, Spanned},
    ir::{Expression, IRInstruction, IRItem, RelativeOffset},
};

use super::super::{InstructionDefinition, RegisterEffect, RegisterSet};

pub(in crate::instructions) const DESUGAR: InstructionDefinition = InstructionDefinition {
    mnemonic: "memcpy",
    handler: desugar,
    effect: RegisterEffect {
        scratch: RegisterSet::from_registers(&[Register::A]),
    },
};

pub(super) fn instruction(value: IRInstruction, span: &Span) -> IRItem {
    IRItem::Instruction(Spanned {
        value,
        span: span.clone(),
    })
}

pub(super) fn label(value: String, span: &Span) -> IRItem {
    IRItem::Label(Spanned {
        value,
        span: span.clone(),
    })
}

pub(super) fn relative_label(value: String, span: &Span) -> RelativeOffset {
    RelativeOffset::Address(Expression::Label(Spanned {
        value,
        span: span.clone(),
    }))
}

pub(super) fn inc16(hi: Register, lo: Register, span: &Span, prefix: &str) -> Vec<IRItem> {
    let inc_hi = format!("{prefix}_hi");
    let end = format!("{prefix}_end");

    vec![
        instruction(IRInstruction::Push { src: Register::A }, span),
        instruction(
            IRInstruction::Ldi {
                dst: Register::A,
                src: Expression::Immediate(0xFF),
            },
            span,
        ),
        instruction(
            IRInstruction::Cmp {
                dst: lo,
                src: Register::A,
            },
            span,
        ),
        instruction(IRInstruction::Pop { dst: Register::A }, span),
        instruction(
            IRInstruction::Jzr {
                offset: relative_label(inc_hi.clone(), span),
            },
            span,
        ),
        instruction(IRInstruction::Push { src: Register::A }, span),
        instruction(
            IRInstruction::Ldi {
                dst: Register::A,
                src: Expression::Immediate(1),
            },
            span,
        ),
        instruction(
            IRInstruction::Add {
                dst: lo,
                src: Register::A,
            },
            span,
        ),
        instruction(IRInstruction::Pop { dst: Register::A }, span),
        instruction(
            IRInstruction::Jr {
                offset: relative_label(end.clone(), span),
            },
            span,
        ),
        label(inc_hi, span),
        instruction(
            IRInstruction::Ldi {
                dst: lo,
                src: Expression::Immediate(0),
            },
            span,
        ),
        instruction(IRInstruction::Push { src: Register::A }, span),
        instruction(
            IRInstruction::Ldi {
                dst: Register::A,
                src: Expression::Immediate(1),
            },
            span,
        ),
        instruction(
            IRInstruction::Add {
                dst: hi,
                src: Register::A,
            },
            span,
        ),
        instruction(IRInstruction::Pop { dst: Register::A }, span),
        label(end, span),
        instruction(IRInstruction::Nop, span),
    ]
}

#[allow(clippy::too_many_lines)]
pub(super) fn desugar(
    instruction_: &ASTInstruction,
    span: &Span,
    id: usize,
) -> Option<Vec<IRItem>> {
    let (
        "memcpy",
        Some(Operand::MemoryWrapped(DataSource::RegisterPair(dst_hi, dst_lo))),
        Some(Operand::MemoryWrapped(DataSource::RegisterPair(src_hi, src_lo))),
        Some(Operand::Raw(DataSource::Register(len))),
        None,
    ) = (
        instruction_.mnemonic.as_str(),
        instruction_.operands.first(),
        instruction_.operands.get(1),
        instruction_.operands.get(2),
        instruction_.operands.get(3),
    )
    else {
        return None;
    };

    let loop_label = format!("__mb8_memcpy_loop_{id}");
    let end_label = format!("__mb8_memcpy_end_{id}");
    let mut items = vec![
        instruction(IRInstruction::Push { src: Register::A }, span),
        instruction(
            IRInstruction::Ldi {
                dst: Register::A,
                src: Expression::Immediate(0),
            },
            span,
        ),
        label(loop_label.clone(), span),
        instruction(IRInstruction::Push { src: Register::A }, span),
        instruction(
            IRInstruction::Ld {
                dst: Register::A,
                hi: *src_hi,
                lo: *src_lo,
            },
            span,
        ),
        instruction(
            IRInstruction::St {
                src: Register::A,
                hi: *dst_hi,
                lo: *dst_lo,
            },
            span,
        ),
        instruction(IRInstruction::Pop { dst: Register::A }, span),
        instruction(
            IRInstruction::Cmp {
                dst: Register::A,
                src: *len,
            },
            span,
        ),
        instruction(
            IRInstruction::Jzr {
                offset: relative_label(end_label.clone(), span),
            },
            span,
        ),
        instruction(IRInstruction::Push { src: Register::R7 }, span),
        instruction(
            IRInstruction::Ldi {
                dst: Register::R7,
                src: Expression::Immediate(1),
            },
            span,
        ),
        instruction(
            IRInstruction::Add {
                dst: Register::A,
                src: Register::R7,
            },
            span,
        ),
        instruction(IRInstruction::Pop { dst: Register::R7 }, span),
    ];
    items.extend(inc16(
        *src_hi,
        *src_lo,
        span,
        &format!("__mb8_memcpy_src_{id}"),
    ));
    items.extend(inc16(
        *dst_hi,
        *dst_lo,
        span,
        &format!("__mb8_memcpy_dst_{id}"),
    ));
    items.extend([
        instruction(
            IRInstruction::Jr {
                offset: relative_label(loop_label, span),
            },
            span,
        ),
        label(end_label, span),
        instruction(IRInstruction::Pop { dst: Register::A }, span),
    ]);

    Some(items)
}
