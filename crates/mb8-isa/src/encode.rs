use crate::{opcodes::Opcode, registers::Register};

/// Encode a Register into a 4-bit value.
#[must_use]
pub fn encode_register(register: Register) -> u8 {
    match register {
        Register::R0 | Register::A => 0x0,
        Register::R1 => 0x1,
        Register::R2 => 0x2,
        Register::R3 => 0x3,
        Register::R4 => 0x4,
        Register::R5 => 0x5,
        Register::R6 => 0x6,
        Register::R7 => 0x7,
        Register::R8 => 0x8,
        Register::R9 | Register::IH => 0x9,
        Register::R10 | Register::IL => 0xA,
        Register::R11 | Register::FPH => 0xB,
        Register::R12 | Register::FPL => 0xC,
        Register::R13 | Register::SPH => 0xD,
        Register::R14 | Register::SPL => 0xE,
        Register::R15 | Register::F => 0xF,
    }
}

/// Encode a Program into a Vec<u8>.
#[must_use]
pub fn encode_program(program: &[Opcode]) -> Vec<u8> {
    program
        .iter()
        .flat_map(|opcode| encode(opcode).to_be_bytes())
        .collect()
}

/// Encode an Opcode into a 16-bit instruction.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn encode(opcode: &Opcode) -> u16 {
    match opcode {
        Opcode::Nop => 0x0000,
        Opcode::Halt => 0x0100,
        Opcode::Sys => 0x0200,
        Opcode::Mov { dst, src } => {
            let dst = encode_register(*dst);
            let src = encode_register(*src);
            0x1000 | (dst as u16) << 4 | src as u16
        }
        Opcode::Add { dst, src } => {
            let dst = encode_register(*dst);
            let src = encode_register(*src);
            0x1100 | (dst as u16) << 4 | src as u16
        }
        Opcode::Sub { dst, src } => {
            let dst = encode_register(*dst);
            let src = encode_register(*src);
            0x1200 | (dst as u16) << 4 | src as u16
        }
        Opcode::And { dst, src } => {
            let dst = encode_register(*dst);
            let src = encode_register(*src);
            0x1300 | (dst as u16) << 4 | src as u16
        }
        Opcode::Or { dst, src } => {
            let dst = encode_register(*dst);
            let src = encode_register(*src);
            0x1400 | (dst as u16) << 4 | src as u16
        }
        Opcode::Xor { dst, src } => {
            let dst = encode_register(*dst);
            let src = encode_register(*src);
            0x1500 | (dst as u16) << 4 | src as u16
        }
        Opcode::Shr { dst, src } => {
            let dst = encode_register(*dst);
            let src = encode_register(*src);
            0x1600 | (dst as u16) << 4 | src as u16
        }
        Opcode::Shl { dst, src } => {
            let dst = encode_register(*dst);
            let src = encode_register(*src);
            0x1700 | (dst as u16) << 4 | src as u16
        }
        Opcode::Cmp { dst, src } => {
            let dst = encode_register(*dst);
            let src = encode_register(*src);
            0x1800 | (dst as u16) << 4 | src as u16
        }
        Opcode::Ldi { dst, value } => {
            let dst = encode_register(*dst);
            0x2000 | (dst as u16) << 4 | *value as u16
        }
        Opcode::Jmp { hi, lo } => {
            let hi = encode_register(*hi);
            let lo = encode_register(*lo);
            0x3000 | (hi as u16) << 4 | lo as u16
        }
        Opcode::Jr { offset } => 0x3100 | (*offset as u8) as u16,
        Opcode::Jzr { offset } => 0x3200 | (*offset as u8) as u16,
        Opcode::Jnzr { offset } => 0x3300 | (*offset as u8) as u16,
        Opcode::Jcr { offset } => 0x3400 | (*offset as u8) as u16,
        Opcode::Jncr { offset } => 0x3500 | (*offset as u8) as u16,
        Opcode::Call { hi, lo } => {
            let hi = encode_register(*hi);
            let lo = encode_register(*lo);
            0x4000 | (hi as u16) << 4 | lo as u16
        }
        Opcode::Ret => 0x4100,
        Opcode::Push { src } => {
            let src = encode_register(*src);
            0x4200 | (src as u16) << 4
        }
        Opcode::Pop { dst } => {
            let dst = encode_register(*dst);
            0x4300 | (dst as u16) << 4
        }
        Opcode::Ld { dst, hi, lo } => {
            let dst = encode_register(*dst);
            let hi = encode_register(*hi);
            let lo = encode_register(*lo);
            0x5000 | (dst as u16) << 8 | (hi as u16) << 4 | lo as u16
        }
        Opcode::St { src, hi, lo } => {
            let src = encode_register(*src);
            let hi = encode_register(*hi);
            let lo = encode_register(*lo);
            0x6000 | (src as u16) << 8 | (hi as u16) << 4 | lo as u16
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_register() {
        assert_eq!(encode_register(Register::R0), 0x0);
        assert_eq!(encode_register(Register::R1), 0x1);
        assert_eq!(encode_register(Register::R2), 0x2);
        assert_eq!(encode_register(Register::R3), 0x3);
        assert_eq!(encode_register(Register::R4), 0x4);
        assert_eq!(encode_register(Register::R5), 0x5);
        assert_eq!(encode_register(Register::R6), 0x6);
        assert_eq!(encode_register(Register::R7), 0x7);
        assert_eq!(encode_register(Register::R8), 0x8);
        assert_eq!(encode_register(Register::R9), 0x9);
        assert_eq!(encode_register(Register::R10), 0xA);
        assert_eq!(encode_register(Register::R11), 0xB);
        assert_eq!(encode_register(Register::R12), 0xC);
        assert_eq!(encode_register(Register::R13), 0xD);
        assert_eq!(encode_register(Register::R14), 0xE);
        assert_eq!(encode_register(Register::R15), 0xF);
    }

    #[test]
    fn test_encode_program() {
        let program = vec![Opcode::Nop, Opcode::Halt];
        assert_eq!(encode_program(&program), vec![0x00, 0x00, 0x01, 0x00]);
    }

    #[test]
    fn test_encode_nop() {
        assert_eq!(encode(&Opcode::Nop), 0x0000);
    }

    #[test]
    fn test_encode_halt() {
        assert_eq!(encode(&Opcode::Halt), 0x0100);
    }

    #[test]
    fn test_encode_syscall() {
        assert_eq!(encode(&Opcode::Sys), 0x0200);
    }

    #[test]
    fn test_encode_mov() {
        assert_eq!(
            encode(&Opcode::Mov {
                dst: Register::R0,
                src: Register::R1
            }),
            0x1001
        );
    }

    #[test]
    fn test_encode_add() {
        assert_eq!(
            encode(&Opcode::Add {
                dst: Register::R0,
                src: Register::R1
            }),
            0x1101
        );
    }

    #[test]
    fn test_encode_sub() {
        assert_eq!(
            encode(&Opcode::Sub {
                dst: Register::R0,
                src: Register::R1
            }),
            0x1201
        );
    }

    #[test]
    fn test_encode_and() {
        assert_eq!(
            encode(&Opcode::And {
                dst: Register::R0,
                src: Register::R1
            }),
            0x1301
        );
    }

    #[test]
    fn test_encode_or() {
        assert_eq!(
            encode(&Opcode::Or {
                dst: Register::R0,
                src: Register::R1
            }),
            0x1401
        );
    }

    #[test]
    fn test_encode_xor() {
        assert_eq!(
            encode(&Opcode::Xor {
                dst: Register::R0,
                src: Register::R1
            }),
            0x1501
        );
    }

    #[test]
    fn test_encode_shr() {
        assert_eq!(
            encode(&Opcode::Shr {
                dst: Register::R0,
                src: Register::R1
            }),
            0x1601
        );
    }

    #[test]
    fn test_encode_shl() {
        assert_eq!(
            encode(&Opcode::Shl {
                dst: Register::R0,
                src: Register::R1
            }),
            0x1701
        );
    }

    #[test]
    fn test_encode_cmp() {
        assert_eq!(
            encode(&Opcode::Cmp {
                dst: Register::R0,
                src: Register::R1
            }),
            0x1801
        );
    }

    #[test]
    fn test_encode_ldi() {
        assert_eq!(
            encode(&Opcode::Ldi {
                dst: Register::R0,
                value: 0x12
            }),
            0x2012
        );
    }

    #[test]
    fn test_encode_jmp() {
        assert_eq!(
            encode(&Opcode::Jmp {
                hi: Register::R1,
                lo: Register::R2
            }),
            0x3012
        );
    }

    #[test]
    fn test_encode_jr() {
        assert_eq!(encode(&Opcode::Jr { offset: 0x23 }), 0x3123);
    }

    #[test]
    fn test_encode_jzr() {
        assert_eq!(encode(&Opcode::Jzr { offset: 0x23 }), 0x3223);
    }

    #[test]
    fn test_encode_jnzr() {
        assert_eq!(encode(&Opcode::Jnzr { offset: 0x23 }), 0x3323);
    }

    #[test]
    fn test_encode_jcr() {
        assert_eq!(encode(&Opcode::Jcr { offset: 0x23 }), 0x3423);
    }

    #[test]
    fn test_encode_jncr() {
        assert_eq!(encode(&Opcode::Jncr { offset: 0x23 }), 0x3523);
    }

    #[test]
    fn test_encode_call() {
        assert_eq!(
            encode(&Opcode::Call {
                hi: Register::R1,
                lo: Register::R2
            }),
            0x4012
        );
    }

    #[test]
    fn test_encode_ret() {
        assert_eq!(encode(&Opcode::Ret), 0x4100);
    }

    #[test]
    fn test_encode_push() {
        assert_eq!(encode(&Opcode::Push { src: Register::R1 }), 0x4210);
    }

    #[test]
    fn test_encode_pop() {
        assert_eq!(encode(&Opcode::Pop { dst: Register::R1 }), 0x4310);
    }

    #[test]
    fn test_encode_ld() {
        assert_eq!(
            encode(&Opcode::Ld {
                dst: Register::R1,
                hi: Register::R2,
                lo: Register::R3
            }),
            0x5123
        );
    }

    #[test]
    fn test_encode_st() {
        assert_eq!(
            encode(&Opcode::St {
                src: Register::R1,
                hi: Register::R2,
                lo: Register::R3
            }),
            0x6123
        );
    }
}
