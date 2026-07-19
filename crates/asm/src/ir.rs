use mb8_isa::registers::Register;

use crate::diagnostics::Spanned;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IRProgram {
    pub origin: Option<Spanned<u16>>,
    pub items: Vec<IRItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IRItem {
    Instruction(Spanned<IRInstruction>),
    Label(Spanned<String>),
    Data(Spanned<Vec<u8>>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IRInstruction {
    Nop,
    Halt {
        code: u8,
    },
    Sys,

    Mov {
        dst: Register,
        src: Register,
    },
    Add {
        dst: Register,
        src: Register,
    },
    Sub {
        dst: Register,
        src: Register,
    },
    And {
        dst: Register,
        src: Register,
    },
    Or {
        dst: Register,
        src: Register,
    },
    Xor {
        dst: Register,
        src: Register,
    },
    Shr {
        dst: Register,
        src: Register,
    },
    Shl {
        dst: Register,
        src: Register,
    },
    Cmp {
        dst: Register,
        src: Register,
    },

    Ldi {
        dst: Register,
        src: Expression,
    },

    Jmp {
        hi: Register,
        lo: Register,
    },
    Jr {
        offset: RelativeOffset,
    },
    Jzr {
        offset: RelativeOffset,
    },
    Jnzr {
        offset: RelativeOffset,
    },
    Jcr {
        offset: RelativeOffset,
    },
    Jncr {
        offset: RelativeOffset,
    },

    Call {
        hi: Register,
        lo: Register,
    },
    Ret,
    Push {
        src: Register,
    },
    Pop {
        dst: Register,
    },

    Ld {
        dst: Register,
        hi: Register,
        lo: Register,
    },
    St {
        src: Register,
        hi: Register,
        lo: Register,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Label(Spanned<String>),
    Immediate(u16),
    Hi(Box<Expression>),
    Lo(Box<Expression>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelativeOffset {
    Immediate(u16),
    Address(Expression),
}
