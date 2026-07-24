use crate::{ast::ASTInstruction, diagnostics::Span, ir::IRItem};

mod add;
mod and;
mod call;
mod cmp;
mod halt;
mod halt_immediate;
mod jcr;
mod jmp;
mod jncr;
mod jnzr;
mod jr;
mod jzr;
mod ld;
mod ldi;
mod mov;
mod nop;
mod or;
mod pop;
mod pseudo;
mod push;
mod ret;
mod shl;
mod shr;
mod st;
mod sub;
mod sys;
mod xor;

type Handler = fn(instruction: &ASTInstruction, span: &Span, id: usize) -> Option<Vec<IRItem>>;

pub(super) struct InstructionDefinition {
    pub mnemonic: &'static str,
    pub handler: Handler,
}

pub(super) const HANDLERS: &[InstructionDefinition] = &[
    add::DESUGAR,
    and::DESUGAR,
    call::DESUGAR,
    pseudo::call_immediate::DESUGAR,
    pseudo::call_label::DESUGAR,
    cmp::DESUGAR,
    pseudo::cmpi::DESUGAR,
    pseudo::dec::DESUGAR,
    halt::DESUGAR,
    halt_immediate::DESUGAR,
    pseudo::inc::DESUGAR,
    pseudo::inc16::DESUGAR,
    jcr::DESUGAR,
    pseudo::jcr_immediate::DESUGAR,
    pseudo::jcr_label::DESUGAR,
    jmp::DESUGAR,
    pseudo::jmp_immediate::DESUGAR,
    pseudo::jmp_label::DESUGAR,
    jncr::DESUGAR,
    pseudo::jncr_immediate::DESUGAR,
    pseudo::jncr_label::DESUGAR,
    jnzr::DESUGAR,
    pseudo::jnzr_immediate::DESUGAR,
    pseudo::jnzr_label::DESUGAR,
    jr::DESUGAR,
    pseudo::jr_immediate::DESUGAR,
    pseudo::jr_label::DESUGAR,
    jzr::DESUGAR,
    pseudo::jzr_immediate::DESUGAR,
    pseudo::jzr_label::DESUGAR,
    ld::DESUGAR,
    pseudo::ld_immediate::DESUGAR,
    pseudo::ld_label::DESUGAR,
    pseudo::ld_offset::DESUGAR,
    ldi::DESUGAR,
    pseudo::ldi_register_pair_immediate::DESUGAR,
    pseudo::ldi_register_pair_label::DESUGAR,
    mov::DESUGAR,
    pseudo::mul::DESUGAR,
    nop::DESUGAR,
    pseudo::not::DESUGAR,
    or::DESUGAR,
    pop::DESUGAR,
    push::DESUGAR,
    ret::DESUGAR,
    shl::DESUGAR,
    pseudo::shli::DESUGAR,
    shr::DESUGAR,
    pseudo::shri::DESUGAR,
    st::DESUGAR,
    pseudo::st_immediate::DESUGAR,
    pseudo::st_label::DESUGAR,
    sub::DESUGAR,
    pseudo::swap::DESUGAR,
    sys::DESUGAR,
    xor::DESUGAR,
    pseudo::zero::DESUGAR,
];
