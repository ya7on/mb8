use mb8_isa::registers::Register;

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn pop(&mut self, dst: Register) {
        let mut stack_pointer = u16::from_be_bytes([
            self.registers.read(Register::SPH),
            self.registers.read(Register::SPL),
        ]);
        stack_pointer += 1;
        if stack_pointer > 0xBFFF {
            self.halted = true;
            return;
        }
        let [sp_hi, sp_lo] = stack_pointer.to_be_bytes();
        let value = self.devices.read(stack_pointer);
        self.registers.write(Register::SPH, sp_hi);
        self.registers.write(Register::SPL, sp_lo);
        self.registers.write(dst, value);
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::opcodes::Opcode;

    use super::*;

    #[test]
    fn pops_value_from_stack() {
        // VM pops a value from the stack
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::SPH, 0xBF);
        vm.registers.write(Register::SPL, 0xFE);
        vm.devices.write(0xBFFF, 0x45);
        vm.execute(&Opcode::Pop { dst: Register::R0 });
        assert_eq!(
            (
                vm.registers.read(Register::SPH),
                vm.registers.read(Register::SPL)
            ),
            (0xBF, 0xFF)
        );
        assert_eq!(vm.registers.read(Register::R0), 0x45);
    }

    #[test]
    fn halts_on_stack_underflow() {
        // VM halts when the stack underflows
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::SPH, 0xBF);
        vm.registers.write(Register::SPL, 0xFF);
        vm.execute(&Opcode::Pop { dst: Register::R0 });
        assert!(vm.halted);
    }
}
