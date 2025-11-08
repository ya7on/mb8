use mb8_isa::{opcodes::Opcode, registers::Register};

const OPCODE_MASK: u16 = 0xF000;
const A_MASK: u16 = 0x0F00;
const B_MASK: u16 = 0x00F0;
const C_MASK: u16 = 0x000F;

pub fn parse_register(reg: u16) -> Option<Register> {
    match reg {
        0x0 => Some(Register::R0),
        0x1 => Some(Register::R1),
        0x2 => Some(Register::R2),
        0x3 => Some(Register::R3),
        0x4 => Some(Register::R4),
        0x5 => Some(Register::R5),
        0x6 => Some(Register::R6),
        0x7 => Some(Register::R7),
        0xD => Some(Register::SP),
        0xE => Some(Register::PC),
        0xF => Some(Register::F),
        _ => None,
    }
}

pub fn parse(instruction: u16) -> Option<Opcode> {
    let opcode = (instruction & OPCODE_MASK) >> 12;
    let a = (instruction & A_MASK) >> 8;
    let b = (instruction & B_MASK) >> 4;
    let c = instruction & C_MASK;
    match opcode {
        0x0 => {
            // Control instructions
            match a {
                0x0 => Some(Opcode::Nop),
                0x1 => Some(Opcode::Halt),
                _ => None,
            }
        }
        0x1 => {
            // Group of reg-reg instructions
            match a {
                0x0 => Some(Opcode::Mov {
                    dst: parse_register(b)?,
                    src: parse_register(c)?,
                }),
                0x1 => Some(Opcode::Add {
                    dst: parse_register(b)?,
                    src: parse_register(c)?,
                }),
                0x2 => Some(Opcode::Sub {
                    dst: parse_register(b)?,
                    src: parse_register(c)?,
                }),
                _ => None,
            }
        }
        _ => None,
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_register() {
        assert_eq!(parse_register(0x0), Some(Register::R0));
        assert_eq!(parse_register(0x1), Some(Register::R1));
        assert_eq!(parse_register(0x2), Some(Register::R2));
        assert_eq!(parse_register(0x3), Some(Register::R3));
        assert_eq!(parse_register(0x4), Some(Register::R4));
        assert_eq!(parse_register(0x5), Some(Register::R5));
        assert_eq!(parse_register(0x6), Some(Register::R6));
        assert_eq!(parse_register(0x7), Some(Register::R7));
        assert_eq!(parse_register(0xD), Some(Register::SP));
        assert_eq!(parse_register(0xE), Some(Register::PC));
        assert_eq!(parse_register(0xF), Some(Register::F));
    }

    #[test]
    fn test_parse_nop() {
        assert_eq!(parse(0x0000), Some(Opcode::Nop));
    }

    #[test]
    fn test_parse_halt() {
        assert_eq!(parse(0x0100), Some(Opcode::Halt));
    }

    #[test]
    fn test_parse_mov() {
        assert_eq!(
            parse(0x1001),
            Some(Opcode::Mov {
                dst: Register::R0,
                src: Register::R1,
            })
        );
    }

    #[test]
    fn test_parse_add() {
        assert_eq!(
            parse(0x1101),
            Some(Opcode::Add {
                dst: Register::R0,
                src: Register::R1,
            })
        );
    }

    #[test]
    fn test_parse_sub() {
        assert_eq!(
            parse(0x1201),
            Some(Opcode::Sub {
                dst: Register::R0,
                src: Register::R1,
            })
        );
    }
}
