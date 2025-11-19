use mb8_isa::{opcodes::Syscall, registers::Register};

use crate::vm::VirtualMachine;

impl VirtualMachine {
    pub fn sys(&mut self, syscall: Syscall, src: Register) {
        match syscall {
            Syscall::Putc => self.sys_putc(src),
        }
    }

    fn sys_putc(&mut self, src: Register) {
        let value = self.registers.read(src);
        println!("{}", value as u8 as char);

        println!("{}", self.registers);
        println!("=");
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn prints_putchar_output() {
        // VM prints the value of the source register
    }
}
