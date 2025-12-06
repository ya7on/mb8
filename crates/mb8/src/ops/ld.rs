use mb8_isa::registers::Register;

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn ld(&mut self, dst: Register, hi: Register, lo: Register) {
        let addr_hi = self.registers.read(hi);
        let addr_lo = self.registers.read(lo);
        let addr = u16::from_be_bytes([addr_hi, addr_lo]);
        let value = self.devices.read(addr);
        self.registers.write(dst, value);
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::opcodes::Opcode;

    use super::*;

    #[test]
    fn loads_byte_from_memory_into_register() {
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::R0, 0x12);
        vm.registers.write(Register::R1, 0x34);
        vm.devices.write(0x1234, 0xAB);

        vm.execute(&Opcode::Ld {
            dst: Register::R2,
            hi: Register::R0,
            lo: Register::R1,
        });

        assert_eq!(vm.registers.read(Register::R2), 0xAB);
    }
}
