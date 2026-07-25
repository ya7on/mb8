use std::fmt;

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
    Constant(Spanned<String>),
    Label(Spanned<String>),
}

impl fmt::Display for DataSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Register(_) => write!(f, "reg"),
            Self::RegisterPair(_, _) => write!(f, "reg:reg"),
            Self::Immediate(_) => write!(f, "imm"),
            Self::Constant(_) => write!(f, "const"),
            Self::Label(_) => write!(f, "label"),
        }
    }
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

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Raw(source) => write!(f, "{source}"),
            Self::MemoryWrapped(source) => write!(f, "[{source}]"),
            Self::MemoryOffset { .. } => write!(f, "[reg:reg - imm]"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Directive {
    Origin(u16),
    Address(u16),
    Include(String),
    Data(Vec<u8>),
    Ascii(String),
    Const { name: Spanned<String>, value: u16 },
}
