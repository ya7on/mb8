#[derive(Debug, Clone)]
pub enum Mb8Asm {
    Import(String),

    Label(String),
    Sublabel(String),

    Push { register: String },
    Pop { register: String },
    St { address: u16, register: String },
    Ld { register: String, address: u16 },
    Ldi { register: String, value: u16 },
    Add { dst: String, src: String },
    Sub { dst: String, src: String },
    Mul { dst: String, src: String },
    Div { dst: String, src: String },

    Call(String),
    Ret,
    Jmp(String),
    Jzr(String),
}

impl ToString for Mb8Asm {
    fn to_string(&self) -> String {
        match self {
            Mb8Asm::Import(name) => format!("#include \"{name}\""),
            Mb8Asm::Label(name) => format!("{name}:"),
            Mb8Asm::Sublabel(name) => format!(".{name}:"),
            Mb8Asm::Push { register } => format!("\tPUSH {register}"),
            Mb8Asm::Pop { register } => format!("\tPOP {register}"),
            Mb8Asm::St { address, register } => format!("\tST [0x{address:X}] {register}"),
            Mb8Asm::Ld { register, address } => format!("\tLD {register} [0x{address:X}]"),
            Mb8Asm::Ldi { register, value } => format!("\tLDI {register} {value}"),
            Mb8Asm::Add { dst, src } => format!("\tADD {dst} {src}"),
            Mb8Asm::Sub { dst, src } => format!("\tSUB {dst} {src}"),
            Mb8Asm::Mul { dst, src } => format!("\tMUL {dst} {src}"),
            Mb8Asm::Div { dst, src } => format!("\tDIV {dst} {src}"),
            Mb8Asm::Call(name) => format!("\tCALL [{name}]"),
            Mb8Asm::Ret => "\tRET".to_string(),
            Mb8Asm::Jmp(name) => format!("\tJMP {}", name),
            Mb8Asm::Jzr(name) => format!("\tJZR {}", name),
        }
    }
}
