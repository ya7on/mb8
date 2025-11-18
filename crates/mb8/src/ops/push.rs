use mb8_isa::{registers::Register, STACK_BOTTOM};

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn push(&mut self, src: Register) {
        let stack_pointer = self.registers.read(Register::SP);
        let value = self.registers.read(src);

        self.devices.write(stack_pointer, value as u8);

        self.registers.write(Register::SP, stack_pointer - 1);

        if stack_pointer - 1 <= STACK_BOTTOM as u16 {
            self.halted = true;
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::opcodes::Opcode;

    use super::*;

    #[test]
    fn test_opcode_push() {
        // VM pushes a value onto the stack
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::R0, 0x45);
        vm.execute(&Opcode::Push { src: Register::R0 });
        assert_eq!(vm.registers.read(Register::SP), 0xBFFF - 1);
        assert_eq!(vm.devices.read(0xBFFF), 0x45);
    }

    #[test]
    fn test_opcode_push_stack_overflow() {
        // VM halts when the stack overflows
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::SP, 0xBF00);
        vm.registers.write(Register::R0, 0x45);
        vm.execute(&Opcode::Push { src: Register::R0 });
        assert!(vm.halted);
    }
}
