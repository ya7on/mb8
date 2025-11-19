use mb8_isa::registers::Register;

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn pop(&mut self, dst: Register) {
        let mut stack_pointer = self.registers.read(Register::SP);
        stack_pointer += 1;
        if stack_pointer > 0xBFFF {
            self.halted = true;
            return;
        }
        let value = self.devices.read(stack_pointer);
        self.registers.write(Register::SP, stack_pointer);
        self.registers.write(dst, value as u16);
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
        vm.registers.write(Register::SP, 0xBFFF - 1);
        vm.devices.write(0xBFFF, 0x45);
        vm.execute(&Opcode::Pop { dst: Register::R0 });
        assert_eq!(vm.registers.read(Register::SP), 0xBFFF);
        assert_eq!(vm.registers.read(Register::R0), 0x45);
    }

    #[test]
    fn halts_on_stack_underflow() {
        // VM halts when the stack underflows
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::SP, 0xBFFF);
        vm.execute(&Opcode::Pop { dst: Register::R0 });
        assert!(vm.halted);
    }
}
