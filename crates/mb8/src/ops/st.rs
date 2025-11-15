use mb8_isa::registers::Register;

use crate::{mem::regions::MemoryRegion, vm::VirtualMachine};

impl VirtualMachine {
    pub fn st(&mut self, src: Register) {
        let addr = self.registers.read(Register::I);
        let value = self.registers.read(src);
        self.mem.general().write(addr, value as u8);
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::opcodes::Opcode;

    use crate::mem::regions::MemoryRegion;

    use super::*;

    #[test]
    fn test_opcode_st() {
        // VM loads data from memory
        let mut vm = VirtualMachine::new();
        vm.registers.write(Register::I, 0x123);
        vm.registers.write(Register::R0, 0x77);
        vm.execute(&Opcode::St { src: Register::R0 });
        assert_eq!(vm.mem.general().read(0x123), 0x77);
    }
}
