//write a debug helper that can be used by both architectures.
use mb8::vm::VirtualMachine;
use mb8_isa::registers::Register;
use minifb::Key;
use std::io::{self, Write};

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

    pub fn print_registers(&mut self, vm: &mut VirtualMachine) {
        let r = &vm.registers;
        println!("=== CURRENT STEP REGISTERS ===");
        println!("R0:  {:02X}", r.read(Register::R0));
        println!("R1:  {:02X}", r.read(Register::R1));
        println!("R2:  {:02X}", r.read(Register::R2));
        println!("R3:  {:02X}", r.read(Register::R3));
        println!("R4:  {:02X}", r.read(Register::R4));
        println!("R5:  {:02X}", r.read(Register::R5));
        println!("R6:  {:02X}", r.read(Register::R6));
        println!("R7:  {:02X}", r.read(Register::R7));
        println!("R8:  {:02X}", r.read(Register::R8));
        println!("R9:  {:02X}", r.read(Register::R9));
        println!("R10:  {:02X}", r.read(Register::R10));
        println!("R11:  {:02X}", r.read(Register::R11));
        println!("R12:  {:02X}", r.read(Register::R12));
        println!("R13:  {:02X}", r.read(Register::R13));
        println!("R14:  {:02X}", r.read(Register::R14));
        println!("R15:  {:02X}", r.read(Register::R15));
    }

    pub fn print_help(&mut self) {
        println!("DEBUGGER COMMANDS");
        println!(" ");
        println!("Commands: ");
        println!("  n  - Step Instruction");
        println!("  c  - Continue Execution");
        println!("  r  - Print Registers");
        println!("  m  - Print Memory");
        println!("  h  - Help");
        println!(" ");
    }

    pub fn handle_debug_byte(&mut self, byte: u8, debug_input: &mut Vec<u8>) -> Option<DebugCmd> {
        match byte {
            b'\n' => {
                println!();
                let _ = io::stdout().flush();
                let cmd = Self::execute_debug_command(debug_input);
                Some(cmd)
            }
            0x08 => {
                debug_input.pop();

                print!("\x08 \x08");
                let _ = io::stdout().flush();
                None
            }
            _ => {
                debug_input.push(byte);
                print!("{}", byte as char);
                let _ = io::stdout().flush();
                None
            }
        }
    }

    fn execute_debug_command(debug_input: &mut [u8]) -> DebugCmd {
        let input = core::str::from_utf8(debug_input).unwrap_or("").trim();
        let mut parts = input.split_whitespace();
        let cmd = parts.next().unwrap_or("");

        match cmd {
            "n" => DebugCmd::Step,
            "c" => DebugCmd::Continue,
            "r" => DebugCmd::Registers,
            "m" => {
                let arg = parts.next().map(ToString::to_string);
                DebugCmd::Memory(arg)
            }
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

    pub fn dump_region_stdout(&mut self, vm: &mut VirtualMachine, start: u16, end: u16) {
        let mut addr = start;
        while addr <= end {
            print!("{addr:04X}: ");
            for i in 0..16 {
                let a = addr + i;
                if a > end {
                    break;
                }
                let val = vm.devices.read(a);
                print!("{val:02X} ");
            }
            println!();
            addr += 16;
        }
    }

    pub fn parse_and_print_memory_stdout(&mut self, arg: &str, vm: &mut VirtualMachine) {
        let arg = arg.trim();

        if arg.is_empty() {
            println!("Usage: m <addr> or m <start>-<end>");
            return;
        }

        if let Some((start_str, end_str)) = arg.split_once('-') {
            let Ok(start) = Self::parse_hex_u16(start_str) else {
                println!("Invalid start address: {start_str}");
                return;
            };

            let Ok(end) = Self::parse_hex_u16(end_str) else {
                println!("Invalid end address: {end_str}");
                return;
            };

            if end < start {
                println!("Invalid range: end < start");
                return;
            }

            println!("=== MEMORY DUMP {start:04X}-{end:04X} ===");

            let max_addr = 0xFFFF;
            let end = end.min(max_addr);

            self.dump_region_stdout(vm, start, end);
        } else {
            let Ok(addr) = Self::parse_hex_u16(arg) else {
                println!("Invalid memory address: {arg}");
                return;
            };

            let max_addr = 0xFFFF;
            if addr > max_addr {
                println!("Address out of bounds: {addr:04X}");
                return;
            }

            let val = vm.devices.read(addr);
            println!("{addr:04X}: {val:02X}");
        }
    }

    fn parse_hex_u16(s: &str) -> Result<u16, ()> {
        let s = s.trim();

        let s = s
            .strip_prefix("0x")
            .or_else(|| s.strip_prefix("0X"))
            .unwrap_or(s);

        u16::from_str_radix(s, 16).map_err(|_| ())
    }

    pub fn print_memory_range_stdout(&mut self, vm: &mut VirtualMachine, start: u16, end: u16) {
        const BYTES_PER_ROW: usize = 16;

        let mut addr = start;

        while addr < end {
            print!("{addr:04X}: ");

            for i in 0..BYTES_PER_ROW {
                let cur = addr.wrapping_add(i as u16);

                if cur < end {
                    let byte = vm.devices.read(cur);
                    print!("{byte:02X} ");
                } else {
                    print!("   ");
                }
            }
            print!("|");
            for i in 0..BYTES_PER_ROW {
                let cur = addr.wrapping_add(i as u16);

                if cur < end {
                    let b = vm.devices.read(cur);
                    let c = if b.is_ascii_graphic() || b == b' ' {
                        b as char
                    } else {
                        '.'
                    };
                    print!("{c}");
                } else {
                    print!(" ");
                }
            }
            println!("|");

            addr = addr.wrapping_add(BYTES_PER_ROW as u16);
        }

        let _ = io::stdout().flush();
    }
}
