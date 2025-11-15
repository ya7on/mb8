use mb8_isa::registers::Register;

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn ret(&mut self) {
        let stack_pointer = self.registers.read(Register::SP);
        let mut stack = self.mem.stack();
        let Ok((addr, stack_pointer)) = stack.pop_u16(stack_pointer) else {
            self.halted = true;
            return;
        };
        self.registers.write(Register::SP, stack_pointer);
        self.registers.write(Register::PC, addr);
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::opcodes::Opcode;

    use crate::mem::regions::MemoryRegion;

    use super::*;

    #[test]
    fn test_opcode_ret() {
        // VM returns from a subroutine
        let mut vm = VirtualMachine::new();
        vm.registers.write(Register::PC, 0x100);
        vm.execute(&Opcode::Call { addr: 0x200 });
        assert_eq!(vm.registers.read(Register::SP), 2);
        assert_eq!(vm.registers.read(Register::PC), 0x200);
        assert_eq!(vm.mem.stack().read(0), 0x01);
        assert_eq!(vm.mem.stack().read(1), 0x00);
        vm.execute(&Opcode::Ret);
        assert_eq!(vm.registers.read(Register::SP), 0);
        assert_eq!(vm.registers.read(Register::PC), 0x100);
    }

    #[test]
    fn test_opcode_ret_stack_underflow() {
        // VM returns from a subroutine
        let mut vm = VirtualMachine::new();
        vm.execute(&Opcode::Ret);
        assert_eq!(vm.registers.read(Register::SP), 0);
        assert_eq!(vm.registers.read(Register::PC), 0);
        assert!(vm.halted);
    }
}
