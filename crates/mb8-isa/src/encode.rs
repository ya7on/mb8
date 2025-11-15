use crate::{
    opcodes::{Opcode, Syscall},
    registers::Register,
};

/// Encode a Register into a 4-bit value.
#[must_use]
pub fn encode_register(register: Register) -> u8 {
    match register {
        Register::R0 => 0x0,
        Register::R1 => 0x1,
        Register::R2 => 0x2,
        Register::R3 => 0x3,
        Register::R4 => 0x4,
        Register::R5 => 0x5,
        Register::R6 => 0x6,
        Register::R7 => 0x7,
        Register::I => 0xC,
        Register::SP => 0xD,
        Register::PC => 0xE,
        Register::F => 0xF,
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
pub fn encode(opcode: &Opcode) -> u16 {
    match opcode {
        Opcode::Nop => 0x0000,
        Opcode::Halt => 0x0100,
        Opcode::Sys { syscall, src } => {
            let syscall = match syscall {
                Syscall::Putc => 0x0,
            };
            let src = encode_register(*src);
            0x0200 | (syscall) << 4 | src as u16
        }
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
        Opcode::Ldi { dst, value } => {
            let dst = encode_register(*dst);
            0x2000 | (dst as u16) << 4 | *value as u16
        }
        Opcode::Jmp { addr } => 0x3000 | (*addr & 0xFFF),
        Opcode::Jz { addr } => 0x4000 | (*addr & 0xFFF),
        Opcode::Jnz { addr } => 0x5000 | (*addr & 0xFFF),
        Opcode::Call { addr } => 0x6000 | (*addr & 0xFFF),
        Opcode::Ret => 0x7000,
        Opcode::Push { src } => {
            let src = encode_register(*src);
            0x7100 | (src as u16) << 4
        }
        Opcode::Pop { dst } => {
            let dst = encode_register(*dst);
            0x7200 | (dst as u16) << 4
        }
        Opcode::LdiI { value } => 0x8000 | (*value & 0xFFF),
        Opcode::Ld { dst } => {
            let dst = encode_register(*dst);
            0x9000 | (dst as u16) << 4
        }
        Opcode::St { src } => {
            let src = encode_register(*src);
            0x9100 | (src as u16) << 4
        }
        Opcode::IncI { src } => {
            let src = encode_register(*src);
            0x9200 | (src as u16) << 4
        }
        Opcode::DecI { src } => {
            let src = encode_register(*src);
            0x9300 | (src as u16) << 4
        }
        Opcode::Draw { x, y, height } => {
            let x = encode_register(*x);
            let y = encode_register(*y);
            0xA000 | (x as u16) << 8 | (y as u16) << 4 | (*height & 0xF) as u16
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
        assert_eq!(encode_register(Register::SP), 0xD);
        assert_eq!(encode_register(Register::PC), 0xE);
        assert_eq!(encode_register(Register::F), 0xF);
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
        assert_eq!(
            encode(&Opcode::Sys {
                syscall: Syscall::Putc,
                src: Register::R1
            }),
            0x0201
        );
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
        assert_eq!(encode(&Opcode::Jmp { addr: 0x123 }), 0x3123);
    }

    #[test]
    fn test_encode_jz() {
        assert_eq!(encode(&Opcode::Jz { addr: 0x123 }), 0x4123);
    }

    #[test]
    fn test_encode_jnz() {
        assert_eq!(encode(&Opcode::Jnz { addr: 0x123 }), 0x5123);
    }

    #[test]
    fn test_encode_call() {
        assert_eq!(encode(&Opcode::Call { addr: 0x123 }), 0x6123);
    }

    #[test]
    fn test_encode_ret() {
        assert_eq!(encode(&Opcode::Ret), 0x7000);
    }

    #[test]
    fn test_encode_push() {
        assert_eq!(encode(&Opcode::Push { src: Register::R1 }), 0x7110);
    }

    #[test]
    fn test_encode_pop() {
        assert_eq!(encode(&Opcode::Pop { dst: Register::R1 }), 0x7210);
    }

    #[test]
    fn test_encode_ldi_i() {
        assert_eq!(encode(&Opcode::LdiI { value: 0x123 }), 0x8123);
    }

    #[test]
    fn test_encode_ld() {
        assert_eq!(encode(&Opcode::Ld { dst: Register::R1 }), 0x9010);
    }

    #[test]
    fn test_encode_st() {
        assert_eq!(encode(&Opcode::St { src: Register::R1 }), 0x9110);
    }

    #[test]
    fn test_encode_inc_i() {
        assert_eq!(encode(&Opcode::IncI { src: Register::R1 }), 0x9210);
    }

    #[test]
    fn test_encode_dec_i() {
        assert_eq!(encode(&Opcode::DecI { src: Register::R1 }), 0x9310);
    }

    #[test]
    fn test_encode_draw() {
        assert_eq!(
            encode(&Opcode::Draw {
                x: Register::R1,
                y: Register::R2,
                height: 0x3
            }),
            0xA123
        );
    }
}
