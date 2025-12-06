use mb8_isa::registers::Register;

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn ret(&mut self) {
        let mut stack_pointer = u16::from_be_bytes([
            self.registers.read(Register::SPH),
            self.registers.read(Register::SPL),
        ]);
        if stack_pointer + 2 > 0xBFFF {
            self.halted = true;
            return;
        }
        stack_pointer += 1;
        let hi = self.devices.read(stack_pointer);
        stack_pointer += 1;
        let lo = self.devices.read(stack_pointer);
        let [sp_hi, sp_lo] = stack_pointer.to_be_bytes();
        self.registers.write(Register::SPH, sp_hi);
        self.registers.write(Register::SPL, sp_lo);
        self.program_counter = u16::from_be_bytes([hi, lo]);
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::opcodes::Opcode;

    use super::*;

    #[test]
    fn restores_pc_from_stack() {
        // VM returns from a subroutine
        let mut vm = VirtualMachine::default();
        vm.program_counter = 0x100;
        vm.registers.write(Register::R0, 0x02);
        vm.registers.write(Register::R1, 0x00);
        vm.execute(&Opcode::Call {
            hi: Register::R0,
            lo: Register::R1,
        });
        assert_eq!(
            (
                vm.registers.read(Register::SPH),
                vm.registers.read(Register::SPL)
            ),
            (0xBF, 0xFD)
        );
        assert_eq!(vm.program_counter, 0x200);
        assert_eq!(vm.devices.read(0xBFFF - 1), 0x01);
        assert_eq!(vm.devices.read(0xBFFF), 0x00);
        vm.execute(&Opcode::Ret);
        assert_eq!(
            (
                vm.registers.read(Register::SPH),
                vm.registers.read(Register::SPL)
            ),
            (0xBF, 0xFF)
        );
        assert_eq!(vm.program_counter, 0x100);
    }

    #[test]
    fn halts_on_ret_underflow() {
        // VM returns from a subroutine
        let mut vm = VirtualMachine::default();
        vm.execute(&Opcode::Ret);
        assert_eq!(
            (
                vm.registers.read(Register::SPH),
                vm.registers.read(Register::SPL)
            ),
            (0xBF, 0xFF)
        );
        assert_eq!(vm.program_counter, 0xE000);
        assert!(vm.halted);
    }
}
