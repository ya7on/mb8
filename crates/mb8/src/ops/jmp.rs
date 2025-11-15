use mb8_isa::{registers::Register, STACK_SIZE};

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn jmp(&mut self, addr: u16) {
        self.registers.write(Register::PC, addr + STACK_SIZE);
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::opcodes::Opcode;

    use super::*;

    #[test]
    fn test_opcode_jmp() {
        // VM jumps to a specific address
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::PC, 0x100);
        vm.execute(&Opcode::Jmp { addr: 0x100 });
        assert_eq!(vm.registers.read(Register::PC), 0x200); // Address is 0x100 + STACK_SIZE
    }
}
