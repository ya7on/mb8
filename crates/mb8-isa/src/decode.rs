use crate::{opcodes::Opcode, registers::Register};

const OPCODE_MASK: u16 = 0xF000;
const A_MASK: u16 = 0x0F00;
const B_MASK: u16 = 0x00F0;
const C_MASK: u16 = 0x000F;

/// Parse a 4-bit register value into a Register enum.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn decode_register(reg: u16) -> Option<Register> {
    match reg {
        0x0 => Some(Register::R0),
        0x1 => Some(Register::R1),
        0x2 => Some(Register::R2),
        0x3 => Some(Register::R3),
        0x4 => Some(Register::R4),
        0x5 => Some(Register::R5),
        0x6 => Some(Register::R6),
        0x7 => Some(Register::R7),
        0x8 => Some(Register::R8),
        0x9 => Some(Register::R9),
        0xA => Some(Register::R10),
        0xB => Some(Register::R11),
        0xC => Some(Register::R12),
        0xD => Some(Register::R13),
        0xE => Some(Register::R14),
        0xF => Some(Register::R15),
        _ => None,
    }
}

/// Decode a 16-bit instruction into an Opcode.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn decode(instruction: u16) -> Option<Opcode> {
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
                0x2 => Some(Opcode::Sys),
                _ => None,
            }
        }
        0x1 => {
            // Group of reg-reg instructions
            match a {
                0x0 => Some(Opcode::Mov {
                    dst: decode_register(b)?,
                    src: decode_register(c)?,
                }),
                0x1 => Some(Opcode::Add {
                    dst: decode_register(b)?,
                    src: decode_register(c)?,
                }),
                0x2 => Some(Opcode::Sub {
                    dst: decode_register(b)?,
                    src: decode_register(c)?,
                }),
                0x3 => Some(Opcode::And {
                    dst: decode_register(b)?,
                    src: decode_register(c)?,
                }),
                0x4 => Some(Opcode::Or {
                    dst: decode_register(b)?,
                    src: decode_register(c)?,
                }),
                0x5 => Some(Opcode::Xor {
                    dst: decode_register(b)?,
                    src: decode_register(c)?,
                }),
                0x6 => Some(Opcode::Shr {
                    dst: decode_register(b)?,
                    src: decode_register(c)?,
                }),
                0x7 => Some(Opcode::Shl {
                    dst: decode_register(b)?,
                    src: decode_register(c)?,
                }),
                0x8 => Some(Opcode::Cmp {
                    dst: decode_register(b)?,
                    src: decode_register(c)?,
                }),
                _ => None,
            }
        }
        0x2 => Some(Opcode::Ldi {
            dst: decode_register(a)?,
            value: (b << 4 | c) as u8,
        }),
        0x3 => match a {
            0x0 => Some(Opcode::Jmp {
                hi: decode_register(b)?,
                lo: decode_register(c)?,
            }),
            0x1 => Some(Opcode::Jr {
                offset: (b << 4 | c) as u8 as i8,
            }),
            0x2 => Some(Opcode::Jzr {
                offset: (b << 4 | c) as u8 as i8,
            }),
            0x3 => Some(Opcode::Jnzr {
                offset: (b << 4 | c) as u8 as i8,
            }),
            0x4 => Some(Opcode::Jcr {
                offset: (b << 4 | c) as u8 as i8,
            }),
            0x5 => Some(Opcode::Jncr {
                offset: (b << 4 | c) as u8 as i8,
            }),
            _ => None,
        },

        0x4 => match a {
            0x0 => Some(Opcode::Call {
                hi: decode_register(b)?,
                lo: decode_register(c)?,
            }),
            0x1 => Some(Opcode::Ret),
            0x2 => Some(Opcode::Push {
                src: decode_register(b)?,
            }),
            0x3 => Some(Opcode::Pop {
                dst: decode_register(b)?,
            }),
            _ => None,
        },
        0x5 => Some(Opcode::Ld {
            dst: decode_register(a)?,
            hi: decode_register(b)?,
            lo: decode_register(c)?,
        }),
        0x6 => Some(Opcode::St {
            src: decode_register(a)?,
            hi: decode_register(b)?,
            lo: decode_register(c)?,
        }),
        _ => None,
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_register() {
        assert_eq!(decode_register(0x0), Some(Register::R0));
        assert_eq!(decode_register(0x1), Some(Register::R1));
        assert_eq!(decode_register(0x2), Some(Register::R2));
        assert_eq!(decode_register(0x3), Some(Register::R3));
        assert_eq!(decode_register(0x4), Some(Register::R4));
        assert_eq!(decode_register(0x5), Some(Register::R5));
        assert_eq!(decode_register(0x6), Some(Register::R6));
        assert_eq!(decode_register(0x7), Some(Register::R7));
        assert_eq!(decode_register(0x8), Some(Register::R8));
        assert_eq!(decode_register(0x9), Some(Register::R9));
        assert_eq!(decode_register(0xA), Some(Register::R10));
        assert_eq!(decode_register(0xB), Some(Register::R11));
        assert_eq!(decode_register(0xC), Some(Register::R12));
        assert_eq!(decode_register(0xD), Some(Register::R13));
        assert_eq!(decode_register(0xE), Some(Register::R14));
        assert_eq!(decode_register(0xF), Some(Register::R15));
    }

    #[test]
    fn test_invalid_instructions() {
        assert_eq!(decode(0xF000), None);

        // Control instructions
        assert_eq!(decode(0x0F00), None);
        // reg-reg instructions
        assert_eq!(decode(0x1F00), None);
        // stack instructions
        assert_eq!(decode(0x9F00), None);
    }

    #[test]
    fn test_parse_nop() {
        assert_eq!(decode(0x0000), Some(Opcode::Nop));
    }

    #[test]
    fn test_parse_halt() {
        assert_eq!(decode(0x0100), Some(Opcode::Halt));
    }

    #[test]
    fn test_parse_syscall_putc() {
        assert_eq!(decode(0x0200), Some(Opcode::Sys));
    }

    #[test]
    fn test_parse_mov() {
        assert_eq!(
            decode(0x1001),
            Some(Opcode::Mov {
                dst: Register::R0,
                src: Register::R1,
            })
        );
    }

    #[test]
    fn test_parse_add() {
        assert_eq!(
            decode(0x1101),
            Some(Opcode::Add {
                dst: Register::R0,
                src: Register::R1,
            })
        );
    }

    #[test]
    fn test_parse_sub() {
        assert_eq!(
            decode(0x1201),
            Some(Opcode::Sub {
                dst: Register::R0,
                src: Register::R1,
            })
        );
    }

    #[test]
    fn test_parse_and() {
        assert_eq!(
            decode(0x1301),
            Some(Opcode::And {
                dst: Register::R0,
                src: Register::R1,
            })
        );
    }

    #[test]
    fn test_parse_or() {
        assert_eq!(
            decode(0x1401),
            Some(Opcode::Or {
                dst: Register::R0,
                src: Register::R1,
            })
        );
    }

    #[test]
    fn test_parse_xor() {
        assert_eq!(
            decode(0x1501),
            Some(Opcode::Xor {
                dst: Register::R0,
                src: Register::R1,
            })
        );
    }

    #[test]
    fn test_parse_shr() {
        assert_eq!(
            decode(0x1601),
            Some(Opcode::Shr {
                dst: Register::R0,
                src: Register::R1,
            })
        );
    }

    #[test]
    fn test_parse_shl() {
        assert_eq!(
            decode(0x1701),
            Some(Opcode::Shl {
                dst: Register::R0,
                src: Register::R1,
            })
        );
    }

    #[test]
    fn test_parse_cmp() {
        assert_eq!(
            decode(0x1801),
            Some(Opcode::Cmp {
                dst: Register::R0,
                src: Register::R1,
            })
        );
    }

    #[test]
    fn test_parse_ldi() {
        assert_eq!(
            decode(0x2069),
            Some(Opcode::Ldi {
                dst: Register::R0,
                value: 0x69,
            })
        );
    }

    #[test]
    fn test_parse_jmp() {
        assert_eq!(
            decode(0x3012),
            Some(Opcode::Jmp {
                hi: Register::R1,
                lo: Register::R2
            })
        );
    }

    #[test]
    fn test_parse_jr() {
        assert_eq!(decode(0x3123), Some(Opcode::Jr { offset: 0x23 }));
    }

    #[test]
    fn test_parse_jzr() {
        assert_eq!(decode(0x3223), Some(Opcode::Jzr { offset: 0x23 }));
    }

    #[test]
    fn test_parse_jnzr() {
        assert_eq!(decode(0x3323), Some(Opcode::Jnzr { offset: 0x23 }));
    }

    #[test]
    fn test_parse_jcr() {
        assert_eq!(decode(0x3423), Some(Opcode::Jcr { offset: 0x23 }));
    }

    #[test]
    fn test_parse_jncr() {
        assert_eq!(decode(0x3523), Some(Opcode::Jncr { offset: 0x23 }));
    }

    #[test]
    fn test_parse_call() {
        assert_eq!(
            decode(0x4012),
            Some(Opcode::Call {
                hi: Register::R1,
                lo: Register::R2
            })
        );
    }

    #[test]
    fn test_parse_ret() {
        assert_eq!(decode(0x4100), Some(Opcode::Ret));
    }

    #[test]
    fn test_parse_push() {
        assert_eq!(decode(0x4210), Some(Opcode::Push { src: Register::R1 }));
    }

    #[test]
    fn test_parse_pop() {
        assert_eq!(decode(0x4310), Some(Opcode::Pop { dst: Register::R1 }));
    }

    #[test]
    fn test_parse_ld() {
        assert_eq!(
            decode(0x5123),
            Some(Opcode::Ld {
                dst: Register::R1,
                hi: Register::R2,
                lo: Register::R3
            })
        );
    }

    #[test]
    fn test_parse_st() {
        assert_eq!(
            decode(0x6123),
            Some(Opcode::St {
                src: Register::R1,
                hi: Register::R2,
                lo: Register::R3
            })
        );
    }
}
