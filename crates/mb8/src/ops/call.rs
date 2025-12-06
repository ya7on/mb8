use mb8_isa::{registers::Register, STACK_BOTTOM};

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn call(&mut self, hi: Register, lo: Register) {
        let mut stack_pointer = u16::from_be_bytes([
            self.registers.read(Register::SPH),
            self.registers.read(Register::SPL),
        ]);
        let program_counter = self.program_counter;

        let hi = self.registers.read(hi);
        let lo = self.registers.read(lo);

        let addr = u16::from_be_bytes([hi, lo]);

        for byte in program_counter.to_le_bytes() {
            self.devices.write(stack_pointer, byte);
            stack_pointer -= 1;

            if stack_pointer as usize <= STACK_BOTTOM {
                self.halted = true;
                return;
            }
        }

        let [sp_hi, sp_lo] = stack_pointer.to_be_bytes();
        self.registers.write(Register::SPH, sp_hi);
        self.registers.write(Register::SPL, sp_lo);
        self.program_counter = addr;
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
        vm.program_counter = 0x9876;
        vm.registers.write(Register::R0, 0x12);
        vm.registers.write(Register::R1, 0x34);
        vm.execute(&Opcode::Call {
            hi: Register::R0,
            lo: Register::R1,
        });
        assert_eq!(
            (
                vm.registers.read(Register::SPH),
                vm.registers.read(Register::SPL)
            ),
            (0xBF, 0xFD),
        );
        assert_eq!(vm.program_counter, 0x1234);
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
