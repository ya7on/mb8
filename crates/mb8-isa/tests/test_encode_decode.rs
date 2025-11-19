use mb8_isa::{
    decode::decode,
    encode::encode,
    opcodes::{Opcode, Syscall},
    registers::Register,
};

#[test]
fn test_round_trip() {
    {
        let opcode = Opcode::Nop {};
        let bin = encode(&opcode);
        assert_eq!(decode(bin), Some(opcode));
    }
    {
        let opcode = Opcode::Halt {};
        let bin = encode(&opcode);
        assert_eq!(decode(bin), Some(opcode));
    }
    {
        let opcode = Opcode::Sys {
            syscall: Syscall::Putc,
            src: Register::R1,
        };
        let bin = encode(&opcode);
        assert_eq!(decode(bin), Some(opcode));
    }
    {
        let opcode = Opcode::Mov {
            dst: Register::R0,
            src: Register::R1,
        };
        let bin = encode(&opcode);
        assert_eq!(decode(bin), Some(opcode));
    }
    {
        let opcode = Opcode::Add {
            dst: Register::R0,
            src: Register::R1,
        };
        let bin = encode(&opcode);
        assert_eq!(decode(bin), Some(opcode));
    }
    {
        let opcode = Opcode::Sub {
            dst: Register::R0,
            src: Register::R1,
        };
        let bin = encode(&opcode);
        assert_eq!(decode(bin), Some(opcode));
    }
    {
        let opcode = Opcode::And {
            dst: Register::R0,
            src: Register::R1,
        };
        let bin = encode(&opcode);
        assert_eq!(decode(bin), Some(opcode));
    }
    {
        let opcode = Opcode::Or {
            dst: Register::R0,
            src: Register::R1,
        };
        let bin = encode(&opcode);
        assert_eq!(decode(bin), Some(opcode));
    }
    {
        let opcode = Opcode::Xor {
            dst: Register::R0,
            src: Register::R1,
        };
        let bin = encode(&opcode);
        assert_eq!(decode(bin), Some(opcode));
    }
    {
        let opcode = Opcode::Shr {
            dst: Register::R0,
            src: Register::R1,
        };
        let bin = encode(&opcode);
        assert_eq!(decode(bin), Some(opcode));
    }
    {
        let opcode = Opcode::Shl {
            dst: Register::R0,
            src: Register::R1,
        };
        let bin = encode(&opcode);
        assert_eq!(decode(bin), Some(opcode));
    }
    {
        let opcode = Opcode::Ldi {
            dst: Register::R0,
            value: 0x12,
        };
        let bin = encode(&opcode);
        assert_eq!(decode(bin), Some(opcode));
    }
    {
        let opcode = Opcode::Jmp {
            hi: Register::R1,
            lo: Register::R2,
        };
        let bin = encode(&opcode);
        assert_eq!(decode(bin), Some(opcode));
    }
    {
        let opcode = Opcode::Jr { offset: 0x12 };
        let bin = encode(&opcode);
        assert_eq!(decode(bin), Some(opcode));
    }
    {
        let opcode = Opcode::Jzr { offset: 0x12 };
        let bin = encode(&opcode);
        assert_eq!(decode(bin), Some(opcode));
    }
    {
        let opcode = Opcode::Jnzr { offset: 0x12 };
        let bin = encode(&opcode);
        assert_eq!(decode(bin), Some(opcode));
    }
    {
        let opcode = Opcode::Jcr { offset: 0x12 };
        let bin = encode(&opcode);
        assert_eq!(decode(bin), Some(opcode));
    }
    {
        let opcode = Opcode::Jncr { offset: 0x12 };
        let bin = encode(&opcode);
        assert_eq!(decode(bin), Some(opcode));
    }
    {
        let opcode = Opcode::Call {
            hi: Register::R1,
            lo: Register::R2,
        };
        let bin = encode(&opcode);
        assert_eq!(decode(bin), Some(opcode));
    }
    {
        let opcode = Opcode::Ret {};
        let bin = encode(&opcode);
        assert_eq!(decode(bin), Some(opcode));
    }
    {
        let opcode = Opcode::Push { src: Register::R1 };
        let bin = encode(&opcode);
        assert_eq!(decode(bin), Some(opcode));
    }
    {
        let opcode = Opcode::Pop { dst: Register::R1 };
        let bin = encode(&opcode);
        assert_eq!(decode(bin), Some(opcode));
    }
    {
        let opcode = Opcode::Ld {
            dst: Register::R1,
            hi: Register::R2,
            lo: Register::R3,
        };
        let bin = encode(&opcode);
        assert_eq!(decode(bin), Some(opcode));
    }
    {
        let opcode = Opcode::St {
            src: Register::R1,
            hi: Register::R2,
            lo: Register::R3,
        };
        let bin = encode(&opcode);
        assert_eq!(decode(bin), Some(opcode));
    }
}
