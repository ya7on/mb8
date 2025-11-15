use mb8_isa::{opcodes::Syscall, registers::Register};

use crate::vm::{Role, VirtualMachine};

impl VirtualMachine {
    pub fn sys(&mut self, syscall: Syscall, src: Register) {
        match syscall {
            Syscall::Putc => self.sys_putc(src),
            Syscall::Yield => self.sys_yield(src),
        }
    }

    fn sys_putc(&mut self, src: Register) {
        let value = self.registers.read(src);
        print!("{}", value as u8 as char);
    }

    fn sys_yield(&mut self, src: Register) {
        match self.role {
            Role::Judge => {
                let bot_id = self.registers.read(src);
                assert!(bot_id < self.bots as u16);
                self.switch_context(Role::Bot(bot_id as u8));
            }
            Role::Bot(_id) => {
                self.registers.switch_context(Role::Judge);
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_opcode_sys_putc() {
        // VM prints the value of the source register
    }
}
