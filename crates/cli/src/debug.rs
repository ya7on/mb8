//write a debug helper that can be used by both architectures.
use crate::tty::Tty;
use mb8::vm::VirtualMachine;
use mb8_isa::registers::Register;
use minifb::Key;

#[derive(Debug)]
pub enum DebugCmd {
    Step,
    Continue,
    Registers,
    Memory,
    Help,
    Invalid,
    None,
}

#[derive(Debug)]
pub struct Debug {}

impl Default for Debug {
    fn default() -> Self {
        Self::new()
    }
}

impl Debug {
    #[must_use]
    pub fn new() -> Debug {
        Self {}
    }

    pub fn poll_command(&mut self, window: &minifb::Window) -> Option<DebugCmd> {
        let keys = window.get_keys_pressed(minifb::KeyRepeat::No);

        if let Some(key) = keys.into_iter().next() {
            return match key {
                Key::N => Some(DebugCmd::Step),
                Key::C => Some(DebugCmd::Continue),
                Key::R => Some(DebugCmd::None),
                Key::H => Some(DebugCmd::Help),
                _ => None,
            };
        }

        None
    }

    // I think this is right, it just not fix in the screen so I need to fix the tty.
    pub fn print_registers(&mut self, vm: &mut VirtualMachine, tty: &mut Tty) {
        let r = &vm.registers;
        Self::tty_write_line(&format!("R0:  {:02X}", r.read(Register::R0)), tty);
        Self::tty_write_line(&format!("R1:  {:02X}", r.read(Register::R1)), tty);
        Self::tty_write_line(&format!("R2:  {:02X}", r.read(Register::R2)), tty);
        Self::tty_write_line(&format!("R3:  {:02X}", r.read(Register::R3)), tty);
        Self::tty_write_line(&format!("R4:  {:02X}", r.read(Register::R4)), tty);
        Self::tty_write_line(&format!("R5:  {:02X}", r.read(Register::R5)), tty);
        Self::tty_write_line(&format!("R6:  {:02X}", r.read(Register::R6)), tty);
        Self::tty_write_line(&format!("R7:  {:02X}", r.read(Register::R7)), tty);
        Self::tty_write_line(&format!("R8:  {:02X}", r.read(Register::R8)), tty);
        Self::tty_write_line(&format!("R9:  {:02X}", r.read(Register::R9)), tty);
        Self::tty_write_line(&format!("R10:  {:02X}", r.read(Register::R10)), tty);
        Self::tty_write_line(&format!("R11:  {:02X}", r.read(Register::R11)), tty);
        Self::tty_write_line(&format!("R12:  {:02X}", r.read(Register::R12)), tty);
        Self::tty_write_line(&format!("R13:  {:02X}", r.read(Register::R13)), tty);
        Self::tty_write_line(&format!("R14:  {:02X}", r.read(Register::R14)), tty);
        Self::tty_write_line(&format!("R15:  {:02X}", r.read(Register::R15)), tty);
    }

    //this is wrong but I will fix it once I can actually read it.
    pub fn print_memory(&mut self, vm: &mut VirtualMachine, tty: &mut Tty) {
        let memory = format!("{:?}\n", vm.devices.read(1000));
        Self::tty_write_line(&memory, tty);
    }

    pub fn print_help(&mut self, tty: &mut Tty) {
        Self::tty_write_line("DEBUGGER COMMANDS", tty);
        Self::tty_write_line("", tty);
        Self::tty_write_line("Commands:", tty);
        Self::tty_write_line("  n  - Step Instruction", tty);
        Self::tty_write_line("  c  - Continue Execution", tty);
        Self::tty_write_line("  r  - Print Registers", tty);
        Self::tty_write_line("  m  - Print Memory", tty);
        Self::tty_write_line("  h  - Help", tty);
        Self::tty_write_line("", tty);
    }

    pub fn handle_debug_byte(
        &mut self,
        byte: u8,
        tty: &mut Tty,
        vm: &mut VirtualMachine,
        debug_input: &mut Vec<u8>,
    ) -> Option<DebugCmd> {
        match byte {
            b'\n' => {
                tty.write_byte(b'\n');
                let cmd = self.execute_debug_command(tty, vm, debug_input);
                Some(cmd)
            }
            0x08 => {
                debug_input.pop();
                tty.write_byte(0x08);
                None
            }
            _ => {
                debug_input.push(byte);
                tty.write_byte(byte);
                None
            }
        }
    }

    fn execute_debug_command(
        &mut self,
        tty: &mut Tty,
        vm: &mut VirtualMachine,
        debug_input: &mut Vec<u8>,
    ) -> DebugCmd {
        let cmd = core::str::from_utf8(debug_input).unwrap_or("").trim();

        match cmd {
            "n" => DebugCmd::Step,
            "c" => DebugCmd::Continue,
            "r" => DebugCmd::Registers,
            "m" => DebugCmd::Memory,
            "h" => DebugCmd::Help,
            _ => DebugCmd::Invalid,
        }
    }

    #[must_use]
    pub fn map_debug_key(key: Key) -> Option<u8> {
        Some(match key {
            Key::A => b'a',
            Key::B => b'b',
            Key::C => b'c',
            Key::N => b'n',
            Key::M => b'm',
            Key::S => b's',
            Key::R => b'r',
            Key::H => b'h',

            Key::Enter => b'\n',
            Key::Backspace => 0x08,

            _ => return None,
        })
    }

    fn tty_write_line(line: &str, tty: &mut Tty) {
        let cols = tty.cols;
        let bytes = line.as_bytes();

        for i in 0..cols {
            if i < bytes.len() {
                tty.write_byte(bytes[i]);
            } else {
                tty.write_byte(b' ');
            }
        }
    }
}
