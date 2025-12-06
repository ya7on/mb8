use mb8_isa::registers::Register;

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn st(&mut self, src: Register, hi: Register, lo: Register) {
        let addr_hi = self.registers.read(hi);
        let addr_lo = self.registers.read(lo);
        let addr = u16::from_be_bytes([addr_hi, addr_lo]);
        let value = self.registers.read(src);
        self.devices.write(addr, value);
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::opcodes::Opcode;

    use super::*;

    #[test]
    fn stores_byte_from_register_to_memory() {
        let mut vm = VirtualMachine::default();
        vm.registers.write(Register::R0, 0x12);
        vm.registers.write(Register::R1, 0x34);
        vm.registers.write(Register::R2, 0xCD);

        vm.execute(&Opcode::St {
            src: Register::R2,
            hi: Register::R0,
            lo: Register::R1,
        });

        assert_eq!(vm.devices.read(0x1234), 0xCD);
    }
}
