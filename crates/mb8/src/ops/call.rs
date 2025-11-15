use mb8_isa::registers::Register;

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn call(&mut self, addr: u16) {
        let stack_pointer = self.registers.read(Register::SP);
        let program_counter = self.registers.read(Register::PC);

        let mut stack = self.mem.stack();
        let Ok(stack_pointer) = stack.push_u16(stack_pointer, program_counter) else {
            self.halted = true;
            return;
        };

        self.registers.write(Register::SP, stack_pointer);
        self.registers.write(Register::PC, addr);
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::{opcodes::Opcode, STACK_SIZE};

    use crate::mem::regions::MemoryRegion;

    use super::*;

    #[test]
    fn test_opcode_call() {
        // VM calls a subroutine at a given address
        let mut vm = VirtualMachine::new();
        vm.registers.write(Register::PC, 0x100);
        vm.execute(&Opcode::Call { addr: 0x200 });
        assert_eq!(vm.registers.read(Register::SP), 2);
        assert_eq!(vm.registers.read(Register::PC), 0x200);
        assert_eq!(vm.mem.stack().read(0), 0x01);
        assert_eq!(vm.mem.stack().read(1), 0x00);
    }

    #[test]
    fn test_opcode_call_stack_overflow() {
        // VM halts when the stack overflows
        let mut vm = VirtualMachine::new();
        vm.registers.write(Register::SP, STACK_SIZE - 2);
        vm.execute(&Opcode::Call { addr: 0x456 });
        assert!(vm.halted);
    }
}
