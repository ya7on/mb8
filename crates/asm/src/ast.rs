use mb8_isa::registers::Register;

use crate::diagnostics::Spanned;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ASTProgram {
    pub items: Vec<ASTItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ASTItem {
    Instruction(Spanned<ASTInstruction>),
    Label(Spanned<String>),
    Directive(Spanned<Directive>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ASTInstruction {
    pub mnemonic: String,
    pub operands: Vec<Operand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataSource {
    Register(Register),
    RegisterPair(Register, Register),
    Immediate(u16),
    Label(Spanned<String>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    Raw(DataSource),
    MemoryWrapped(DataSource),
    MemoryOffset {
        hi: Register,
        lo: Register,
        offset: u8,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Directive {
    Origin(u16),
    Address(u16),
    Include(String),
    Data(Vec<u8>),
    Ascii(String),
}
