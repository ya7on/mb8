use mb8_isa::registers::Register;

use crate::{mem::regions::MemoryRegion, vm::VirtualMachine};

impl VirtualMachine {
    pub fn ld(&mut self, dst: Register) {
        let addr = self.registers.read(Register::I);
        let value = self.mem.general().read(addr);
        self.registers.write(dst, value as u16);
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::opcodes::Opcode;

    use crate::mem::regions::MemoryRegion;

    use super::*;

    #[test]
    fn test_opcode_ld() {
        // VM loads data from memory
        let mut vm = VirtualMachine::default();
        vm.mem.general().write(0x123, 0x77);
        vm.registers.write(Register::I, 0x123);
        vm.execute(&Opcode::Ld { dst: Register::R0 });
        assert_eq!(vm.registers.read(Register::R0), 0x77);
    }
}
