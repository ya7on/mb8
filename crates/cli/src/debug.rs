use std::fmt::format;

//write a debug helper that can be used by both architectures.
use crate::tty::Tty;
use mb8::{dev::bus::{self, Bus}, vm::VirtualMachine};
use minifb::Key;

#[derive(Debug)]
pub enum DebugCmd {
    Step,
    Continue,
    Registers,
    Help,
}

#[derive(Debug)]
pub struct Debug {}

impl Debug {
    pub fn new() -> Debug {
        Self {}
    }

    pub fn poll_command(&mut self, window: &minifb::Window) -> Option<DebugCmd> {
        let keys = window.get_keys_pressed(minifb::KeyRepeat::No);

        for key in keys {
            return match key {
                Key::N => Some(DebugCmd::Step),
                Key::C => Some(DebugCmd::Continue),
                Key::R => Some(DebugCmd::Registers),
                Key::H => Some(DebugCmd::Help),
                _ => None,
            };
        }

        None
    }

    pub fn execute_next_instruction() {
        //execute the next instruction
    }

    pub fn continue_normal_execution() {

    }

    // I think this is right, it just not fix in the screen so I need to fix the tty.
    pub fn print_registers(&mut self, vm: &mut VirtualMachine, tty: &mut Tty) {
        let regs = format!("{:?}\n", vm.registers);
        self.tty_write_line(&regs, tty);
    }

    //this is wrong but I will fix it once I can actually read it.
    pub fn print_memory(&mut self, vm: &mut VirtualMachine, tty: &mut Tty) {
        let memory = format!("{:?}\n", vm.devices.read(1000));
        self.tty_write_line(&memory, tty);

    }

    pub fn print_help(&mut self, tty: &mut Tty) {
        self.tty_write_line("DEBUGGER COMMANDS", tty);
        self.tty_write_line("", tty);
        self.tty_write_line("Commands:", tty);
        self.tty_write_line("  n  - Step Instruction", tty);
        self.tty_write_line("  c  - Continue Execution", tty);
        self.tty_write_line("  r  - Print Registers", tty);
        self.tty_write_line("  m  - Print Memory", tty);
        self.tty_write_line("  h  - Help", tty);
        self.tty_write_line("", tty);
    }

    pub fn handle_debug_byte(
        &mut self,
        byte: u8,
        tty: &mut Tty,
        vm: &mut VirtualMachine,
        debug_input: &mut Vec<u8>,
    ) {
        match byte {
            b'\n' => {
                tty.write_byte(b'\n');
                self.execute_debug_command(tty, vm, debug_input);
            }
            0x08 => {
                debug_input.pop();
                tty.write_byte(0x08);
            }
            _ => {
                debug_input.push(byte);
                tty.write_byte(byte);
            }
        }
    }

    fn execute_debug_command(
        &mut self,
        tty: &mut Tty,
        vm: &mut VirtualMachine,
        debug_input: &mut Vec<u8>,
    ) {
        let cmd = core::str::from_utf8(&debug_input).unwrap_or("").trim();

        match cmd {
            "n" => {
                vm.step();
                self.print_help(tty);
            }
            "c" => {
                //self.paused = false;
                //self.debug_prompt = false;
            }
            "r" => {
                self.print_registers(vm, tty);
            }
            "m" => {
                self.print_memory(vm, tty);
            }
            "h" => {
                self.print_help(tty);
            }
            _ => {
                tty.write_byte(b'?');
                tty.write_byte(b'\n');
                self.print_help(tty);
            }
        }

        debug_input.clear();
    }

    pub fn map_debug_key(key: Key) -> Option<u8> {
        Some(match key {
            Key::A => b'a',
            Key::B => b'b',
            Key::C => b'c',
            Key::S => b's',
            Key::R => b'r',
            Key::H => b'h',

            Key::Enter => b'\n',
            Key::Backspace => 0x08,

            _ => return None,
        })
    }

    fn tty_write_line(&mut self, line: &str, tty: &mut Tty) {
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
