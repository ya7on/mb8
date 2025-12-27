use std::fmt::{self, Display};

#[derive(Debug, Clone)]
pub enum Mb8Asm {
    Import(String),

    Label(String),
    Sublabel(String),

    Push { register: String },
    Pop { register: String },
    St { address: u16, register: String },
    Ld { register: String, address: u16 },
    Ldi { register: String, value: u8 },
    Mov { dst: String, src: String },
    Cmp { dst: String, src: String },
    Add { dst: String, src: String },
    Sub { dst: String, src: String },
    Mul { dst: String, src: String },
    Div { dst: String, src: String },
    Inc { register: String },
    Dec { register: String },

    Call(String),
    Ret,
    Jmp(String),
    Jzr(String),
    Jnzr(String),
    Jncr(String),
}

impl Display for Mb8Asm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mb8Asm::Import(name) => write!(f, "#include \"{name}\""),
            Mb8Asm::Label(name) => write!(f, "{name}:"),
            Mb8Asm::Sublabel(name) => write!(f, ".{name}:"),
            Mb8Asm::Push { register } => write!(f, "\tPUSH {register}"),
            Mb8Asm::Pop { register } => write!(f, "\tPOP {register}"),
            Mb8Asm::St { address, register } => write!(f, "\tST [0x{address:X}] {register}"),
            Mb8Asm::Ld { register, address } => write!(f, "\tLD {register} [0x{address:X}]"),
            Mb8Asm::Ldi { register, value } => write!(f, "\tLDI {register} {value}"),
            Mb8Asm::Mov { dst, src } => write!(f, "\tMOV {dst} {src}"),
            Mb8Asm::Cmp { dst, src } => write!(f, "\tCMP {dst} {src}"),
            Mb8Asm::Add { dst, src } => write!(f, "\tADD {dst} {src}"),
            Mb8Asm::Sub { dst, src } => write!(f, "\tSUB {dst} {src}"),
            Mb8Asm::Mul { dst, src } => write!(f, "\tMUL {dst} {src}"),
            Mb8Asm::Div { dst, src } => write!(f, "\tDIV {dst} {src}"),
            Mb8Asm::Inc { register } => write!(f, "\tINC {register}"),
            Mb8Asm::Dec { register } => write!(f, "\tDEC {register}"),
            Mb8Asm::Call(name) => write!(f, "\tCALL [{name}]"),
            Mb8Asm::Ret => f.write_str("\tRET"),
            Mb8Asm::Jmp(name) => write!(f, "\tJMP [{name}]"),
            Mb8Asm::Jzr(name) => write!(f, "\tJZR [{name}]"),
            Mb8Asm::Jnzr(name) => write!(f, "\tJNZR [{name}]"),
            Mb8Asm::Jncr(name) => write!(f, "\tJNCR [{name}]"),
        }
    }
}
