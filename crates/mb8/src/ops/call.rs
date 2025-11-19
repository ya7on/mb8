use mb8_isa::{registers::Register, STACK_BOTTOM};

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn call(&mut self, hi: Register, lo: Register) {
        let mut stack_pointer = self.registers.read(Register::SP);
        let program_counter = self.registers.read(Register::PC);

        let hi = self.registers.read(hi) as u8;
        let lo = self.registers.read(lo) as u8;

        let addr = u16::from_be_bytes([hi, lo]);

        for byte in program_counter.to_le_bytes() {
            self.devices.write(stack_pointer, byte);
            stack_pointer -= 1;

            if stack_pointer as usize <= STACK_BOTTOM {
                self.halted = true;
                return;
            }
        }

        self.registers.write(Register::SP, stack_pointer);
        self.registers.write(Register::PC, addr);
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::opcodes::Opcode;

    use super::*;

    #[test]
    fn pushes_return_and_jumps() {
        // VM calls a subroutine at a given address
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::PC, 0x9876);
        vm.registers.write(Register::R0, 0x12);
        vm.registers.write(Register::R1, 0x34);
        vm.execute(&Opcode::Call {
            hi: Register::R0,
            lo: Register::R1,
        });
        assert_eq!(
            vm.registers.read(Register::SP),
            0xBFFF - 2,
            "SP=0x{:X}",
            vm.registers.read(Register::SP)
        );
        assert_eq!(vm.registers.read(Register::PC), 0x1234);
        assert_eq!(vm.devices.read(0xBFFF - 1), 0x98);
        assert_eq!(vm.devices.read(0xBFFF), 0x76);
    }

    // #[test]
    // fn test_opcode_call_stack_overflow() {
    //     // VM halts when the stack overflows
    //     let mut vm = VirtualMachine::default();
    //     vm.registers.write(Register::SP, 0xBF00);
    //     vm.execute(&Opcode::Call {
    //         hi: Register::R0,
    //         lo: Register::R1,
    //     });
    //     assert!(vm.halted);
    // }
}
