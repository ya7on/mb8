use mb8_isa::{decode::decode, opcodes::Opcode, registers::Register};

use crate::{dev::bus::Bus, registers::Registers};

/// MB8 Virtual Machine
#[derive(Debug, Default)]
pub struct VirtualMachine {
    pub devices: Bus,
    pub registers: Registers,
    pub halted: bool,
}

impl VirtualMachine {
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
            Opcode::Ld { dst, hi, lo } => self.ld(*dst, *hi, *lo),
            Opcode::Jmp { hi, lo } => self.jmp(*hi, *lo),
            Opcode::Jr { offset } => self.jr(*offset),
            Opcode::Jzr { offset } => self.jzr(*offset),
            Opcode::Jnzr { offset } => self.jnzr(*offset),
            Opcode::Jcr { offset } => self.jcr(*offset),
            Opcode::Jncr { offset } => self.jncr(*offset),
            Opcode::Call { hi, lo } => self.call(*hi, *lo),
            Opcode::Ret => self.ret(),
            Opcode::Push { src } => self.push(*src),
            Opcode::Pop { dst } => self.pop(*dst),
            Opcode::St { src, hi, lo } => self.st(*src, *hi, *lo),
        }
    }

    pub fn step(&mut self) {
        let pc = self.registers.read(Register::PC);
        self.registers.write(Register::PC, pc.saturating_add(2));

        let hi = self.devices.read(pc);
        let lo = self.devices.read(pc + 1);
        let binary_instruction = u16::from_be_bytes([hi, lo]);
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

    pub fn load_rom(&mut self, rom: &[u8]) {
        for (i, &byte) in rom.iter().enumerate() {
            self.devices.write((0xE000 + i) as u16, byte);
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // TODO
    // fn test_vm() {
    //     let mut vm = VirtualMachine::default();
    //     vm.load_rom(&[0x00, 0x00, 0x01, 0x00]);
    //     vm.run();
    //     assert_eq!(vm.registers.read(Register::PC), 4);
    // }

    // #[test]
    // fn test_end_of_memory() {
    //     let mut vm = VirtualMachine::default();
    //     vm.registers.write(Register::PC, 4095);
    //     vm.step();
    //     assert!(vm.halted);
    // }

    // TODO
    // #[test]
    // fn test_invalid_opcode() {
    //     let mut vm = VirtualMachine::default();
    //     vm.load_rom(&[0xFF]);
    //     vm.step();
    //     assert!(vm.halted);
    // }
}
