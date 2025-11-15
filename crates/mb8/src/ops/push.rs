use mb8_isa::registers::Register;

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn push(&mut self, src: Register) {
        let stack_pointer = self.registers.read(Register::SP);
        let value = self.registers.read(src);
        let mut stack = self.mem.stack();
        let Ok(stack_pointer) = stack.push_u8(stack_pointer, value as u8) else {
            self.halted = true;
            return;
        };
        self.registers.write(Register::SP, stack_pointer);
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::{opcodes::Opcode, STACK_SIZE};

    use crate::mem::regions::MemoryRegion;

    use super::*;

    #[test]
    fn test_opcode_push() {
        // VM pushes a value onto the stack
        let mut vm = VirtualMachine::new();
        vm.registers.write(Register::R0, 0x45);
        vm.execute(&Opcode::Push { src: Register::R0 });
        assert_eq!(vm.registers.read(Register::SP), 1);
        assert_eq!(vm.mem.stack().read(0), 0x45);
    }

    #[test]
    fn test_opcode_push_stack_overflow() {
        // VM halts when the stack overflows
        let mut vm = VirtualMachine::new();
        vm.registers.write(Register::SP, STACK_SIZE - 1);
        vm.registers.write(Register::R0, 0x45);
        vm.execute(&Opcode::Push { src: Register::R0 });
        assert!(vm.halted);
    }
}
