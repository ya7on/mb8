use mb8_isa::registers::Register;

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn pop(&mut self, dst: Register) {
        let stack_pointer = self.registers.read(Register::SP);
        let mut stack = self.mem.stack();
        let Ok((value, stack_pointer)) = stack.pop_u8(stack_pointer) else {
            self.halted = true;
            return;
        };
        self.registers.write(Register::SP, stack_pointer);
        self.registers.write(dst, value as u16);
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::opcodes::Opcode;

    use crate::mem::regions::MemoryRegion;

    use super::*;

    #[test]
    fn test_opcode_pop() {
        // VM pops a value from the stack
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::SP, 1);
        vm.mem.stack().write(0, 0x45);
        vm.execute(&Opcode::Pop { dst: Register::R0 });
        assert_eq!(vm.registers.read(Register::SP), 0);
        assert_eq!(vm.registers.read(Register::R0), 0x45);
    }

    #[test]
    fn test_opcode_pop_stack_underflow() {
        // VM halts when the stack underflows
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::SP, 0);
        vm.execute(&Opcode::Pop { dst: Register::R0 });
        assert!(vm.halted);
    }
}
