//write a debug helper that can be used by both architectures.
use crate::tty::Tty;
use mb8::vm::VirtualMachine;
use mb8_isa::registers::Register;
use minifb::Key;

const DUMP_ROWS: usize = 16;

#[derive(Debug)]
pub enum DebugCmd {
    Step,
    Continue,
    Registers,
    Memory(Option<String>),
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

    pub fn print_registers(&mut self, vm: &mut VirtualMachine, tty: &mut Tty) {
        let r = &vm.registers;
        Self::tty_write_line("=== CURRENT STEP REGISTERS ===", tty);
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
        debug_input: &mut Vec<u8>,
    ) -> Option<DebugCmd> {
        match byte {
            b'\n' => {
                tty.write_byte(b'\n');
                let cmd = Self::execute_debug_command(debug_input);
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

    fn execute_debug_command(debug_input: &mut Vec<u8>) -> DebugCmd {
        let input = core::str::from_utf8(debug_input).unwrap_or("").trim();
        let mut parts = input.split_whitespace();
        let cmd = parts.next().unwrap_or("");

        match cmd {
            "n" => DebugCmd::Step,
            "c" => DebugCmd::Continue,
            "r" => DebugCmd::Registers,
            "m" => {
                let arg = parts.next().map(|s| s.to_string());
                DebugCmd::Memory(arg)
            }
            "h" => DebugCmd::Help,
            _ => DebugCmd::Invalid,
        }
    }
    //seperating debug completely from the client
    //so you would complete independent functionality , as much as possible.
    #[must_use]
    pub fn map_debug_key(key: Key) -> Option<u8> {
        Some(match key {
            Key::A => b'a',
            Key::B => b'b',
            Key::C => b'c',
            Key::D => b'd',
            Key::E => b'e',
            Key::F => b'f',
            Key::N => b'n',
            Key::M => b'm',
            Key::S => b's',
            Key::R => b'r',
            Key::H => b'h',
            Key::X => b'x',

            Key::Key0 => b'0',
            Key::Key1 => b'1',
            Key::Key2 => b'2',
            Key::Key3 => b'3',
            Key::Key4 => b'4',
            Key::Key5 => b'5',
            Key::Key6 => b'6',
            Key::Key7 => b'7',
            Key::Key8 => b'8',
            Key::Key9 => b'9',

            Key::Space => b' ',
            Key::Minus => b'-',

            Key::Enter => b'\n',
            Key::Backspace => 0x08,

            _ => return None,
        })
    }

    pub fn dump_region(&mut self, tty: &mut Tty, vm: &mut VirtualMachine, start: u16, end: u16) {
        let mut lines_printed = 0;

        let mut addr = start;
        while addr <= end {
            if lines_printed >= DUMP_ROWS {
                Self::tty_write_line("-- more --", tty);
                return; // stop here
            }

            let mut line = format!("{addr:04X}: ");

            for i in 0..16 {
                let a = addr + i;
                if a > end {
                    break;
                }
                let val = vm.devices.read(a);
                line.push_str(&format!("{val:02X} "));
            }

            Self::tty_write_line(&line, tty);

            addr += 16;
            lines_printed += 1;
        }
    }

    fn tty_write_line(line: &str, tty: &mut Tty) {
        for b in line.bytes() {
            tty.write_byte(b);
        }
        tty.write_byte(b'\n');
    }

    pub fn print_memory_range(
        &mut self,
        vm: &mut VirtualMachine,
        tty: &mut Tty,
        start: u16,
        end: u16,
    ) {
        tty.clear();
        tty.reset_stream();
        Self::tty_write_line(
            &format!("=== MEMORY DUMP {start:04X}-{end:04X} ==="),
            tty,
        );
        self.dump_region(tty, vm, start, end);
    }

    pub fn print_memory_addr(&mut self, vm: &mut VirtualMachine, tty: &mut Tty, addr: u16) {
        tty.clear();
        tty.reset_stream();
        let val = vm.devices.read(addr);
        Self::tty_write_line(&format!("Address {addr:04X}: {val:02X}"), tty);
    }

    pub fn parse_and_print_memory(&mut self, arg: &str, vm: &mut VirtualMachine, tty: &mut Tty) {
        if let Some((start_str, end_str)) = arg.split_once('-') {
            if let (Ok(start), Ok(end)) = (
                u16::from_str_radix(start_str.trim_start_matches("0x"), 16),
                u16::from_str_radix(end_str.trim_start_matches("0x"), 16),
            ) {
                self.print_memory_range(vm, tty, start, end);
            } else {
                Self::tty_write_line("Invalid memory range.", tty);
            }
        } else {
            // Single address
            if let Ok(addr) = u16::from_str_radix(arg.trim_start_matches("0x"), 16) {
                self.print_memory_addr(vm, tty, addr);
            } else {
                Self::tty_write_line("Invalid memory address.", tty);
            }
        }
    }
}
