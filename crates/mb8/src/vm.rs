use mb8_isa::{decode::decode, opcodes::Opcode, registers::Register, MEMORY_BANK_SIZE};

use crate::{
    mem::{regions::MemoryRegion, Memory},
    registers::Registers,
};

/// MB8 Virtual Machine
#[derive(Debug)]
pub struct VirtualMachine {
    pub mem: Memory,
    pub registers: Registers,
    pub halted: bool,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            mem: Memory::default(),
            registers: Registers::default(),
            halted: false,
        }
    }

    /// Load memory into the virtual machine.
    pub fn load_rom(&mut self, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            self.mem.rom().write(i as u16, byte);
        }
    }

    /// Execute a single instruction.
    pub fn execute(&mut self, instruction: &Opcode) {
        match instruction {
            Opcode::Nop => self.nop(),
            Opcode::Halt => self.halt(),
            Opcode::Sys { syscall, src } => self.sys(*syscall, *src),
            Opcode::Mov { dst, src } => self.mov(*dst, *src),
            Opcode::Add { dst, src } => self.add(*dst, *src),
            Opcode::Sub { dst, src } => self.sub(*dst, *src),
            Opcode::And { dst, src } => self.and(*dst, *src),
            Opcode::Or { dst, src } => self.or(*dst, *src),
            Opcode::Xor { dst, src } => self.xor(*dst, *src),
            Opcode::Shr { dst, src } => self.shr(*dst, *src),
            Opcode::Shl { dst, src } => self.shl(*dst, *src),
            Opcode::Ldi { dst, value } => self.ldi(*dst, *value),
            Opcode::Jmp { addr } => self.jmp(*addr),
            Opcode::Jz { addr } => self.jz(*addr),
            Opcode::Jnz { addr } => self.jnz(*addr),
            Opcode::Call { addr } => self.call(*addr),
            Opcode::Ret => self.ret(),
            Opcode::Push { src } => self.push(*src),
            Opcode::Pop { dst } => self.pop(*dst),
            Opcode::LdiI { value } => self.ldi_i(*value),
            Opcode::Ld { dst } => self.ld(*dst),
            Opcode::St { src } => self.st(*src),
            Opcode::IncI { src } => self.inc_i(*src),
            Opcode::DecI { src } => self.dec_i(*src),
            Opcode::Draw { x, y, height } => self.draw(*x, *y, *height),
        }
    }

    pub fn step(&mut self) {
        let pc = self.registers.read(Register::PC);
        self.registers.write(Register::PC, pc.saturating_add(2));

        if usize::from(pc) >= MEMORY_BANK_SIZE - 1 {
            self.halted = true;
            return;
        }

        let rom = self.mem.rom();
        let binary_instruction = rom.next_instruction(pc);
        let Some(opcode) = decode(binary_instruction) else {
            self.halted = true;
            return;
        };

        println!("{pc}:\t({binary_instruction:?})");
        println!("{opcode:?}");
        println!("{}", self.registers);
        println!("=");

        self.execute(&opcode);
    }

    /// Execute a program.
    pub fn run(&mut self) {
        while !self.halted {
            self.step();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm() {
        let mut vm = VirtualMachine::new();
        vm.load_rom(&[0x00, 0x00, 0x01, 0x00]);
        vm.run();
        assert_eq!(vm.registers.read(Register::PC), 4);
    }

    #[test]
    fn test_end_of_memory() {
        let mut vm = VirtualMachine::new();
        vm.registers.write(Register::PC, 4095);
        vm.step();
        assert!(vm.halted);
    }

    #[test]
    fn test_invalid_opcode() {
        let mut vm = VirtualMachine::new();
        vm.load_rom(&[0xFF]);
        vm.step();
        assert!(vm.halted);
    }
}
