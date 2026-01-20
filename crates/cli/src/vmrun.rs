use crate::{filesystem::makefs, keyboard::Keyboard};
use std::path::PathBuf;

use mb8::vm;
use minifb::{Window, WindowOptions};

use crate::debug::{Debug, DebugCmd};
use crate::tty::Tty;

const OPS_PER_FRAME: u32 = 1024;
const RENDER_INTERVAL: u32 = 1000;

#[derive(Debug)]
pub enum ScreenMode {
    Vm,
    Debug,
}

#[derive(Debug)]
pub struct VmRun {
    pub vm: vm::VirtualMachine,
    pub tty: Tty,
    pub window: Window,
    ticks: u32,
    width: usize,
    height: usize,
    debug: Debug,
    debug_input: Vec<u8>,
    pub debug_enabled: bool,
    pub hit_entry_break: bool,
    pub paused: bool,
    pub debug_tty: Tty,
    pub screen_mode: ScreenMode,
}

impl VmRun {
    #[must_use]
    pub fn new(vm: vm::VirtualMachine, tty: Tty, debug: Debug) -> Result<Self, minifb::Error> {
        let window = Window::new("MB8", 640, 480, WindowOptions::default())?;

        let debug_tty = Tty::new(40, 25, 64000);

        Ok(Self {
            vm,
            tty,
            window,
            ticks: 0,
            width: 320,
            height: 200,
            debug,
            debug_input: Vec::new(),
            debug_enabled: false,
            hit_entry_break: false,
            paused: false,
            debug_tty,
            screen_mode: ScreenMode::Vm,
        })
    }

    pub fn run_desktop(&mut self, kernel: PathBuf, user: Vec<PathBuf>, seed: Option<u16>) {
        let Ok(rom) = std::fs::read(kernel) else {
            return;
        };
        self.vm.load_rom(&rom);

        let seed = seed.unwrap_or(1);

        self.vm.devices.rand().number = (seed as u8).max(1);

        makefs(user, &mut self.vm);

        self.ticks = RENDER_INTERVAL - 1;
        let l_shift = false;
        let r_shift = false;
        let key = &mut Keyboard::new(l_shift, r_shift);

        while self.window.is_open() && !self.vm.halted {
            match self.screen_mode {
                ScreenMode::Vm => {
                    Keyboard::key_pressed(key, &self.window, &mut self.vm);
                    Keyboard::key_released(key, &self.window);
                    self.render();
                    self.vm_step();
                    self.run_debug();

                    println!("debug enabled: {}", self.debug_enabled);

                    if self.debug_enabled {
                        self.run_debug();
                    }
                }

                ScreenMode::Debug => {
                    self.render_debug();
                    self.poll_debug_keys();
                }
            }
        }
    }

    fn vm_step(&mut self) {
        if self.debug_enabled {
            if !self.vm.halted {
                self.vm.step();
            }
            return;
        }
        for _ in 0..OPS_PER_FRAME {
            if self.vm.halted {
                break;
            }

            self.vm.step();
            println!("PC = {:04X}", self.vm.program_counter);
        }
    }

    fn run_debug(&mut self) -> bool {
        const USER_ENTRY: u16 = 0xE100;
        if self.debug_enabled && !self.hit_entry_break && self.vm.program_counter == USER_ENTRY {
            self.hit_entry_break = true;
            //
            self.paused = true;
            self.screen_mode = ScreenMode::Debug;
        }

        if !self.paused {
            return false;
        }

        if self.paused {
            self.debug_tty.clear();
            self.debug_tty.reset_stream();
            self.debug.print_help(&mut self.debug_tty);
        }
        true
    }

    fn render(&mut self) {
        let mut buf = vec![0u32; self.width * self.height];

        self.tty.load_from_slice(self.vm.devices.gpu().tty_buffer());

        self.tty.render(&mut buf, self.width);

        let _ = self.window.update_with_buffer(&buf, 320, 200);
    }

    fn render_debug(&mut self) {
        let mut buf = vec![0u32; self.width * self.height];

        self.tty.load_from_slice(self.vm.devices.gpu().tty_buffer());

        self.debug_tty.render(&mut buf, self.width);

        let _ = self.window.update_with_buffer(&buf, 320, 200);
    }

    fn poll_debug_keys(&mut self) {
        for key in self.window.get_keys_pressed(minifb::KeyRepeat::No) {
            if let Some(byte) = Debug::map_debug_key(key) {
                if let Some(cmd) = self.debug.handle_debug_byte(
                    byte,
                    &mut self.debug_tty,
                    &mut self.vm,
                    &mut self.debug_input,
                ) {
                    self.apply_debug_cmd(&cmd);
                    self.debug_input.clear();

                    self.debug_tty.write_byte(b'\n');
                    self.debug_tty.write_byte(b'>');
                    self.debug_tty.write_byte(b' ');
                }
            }
        }
    }

    pub fn execute_next_instruction(&mut self) {
        if self.vm.halted {
            return;
        }

        self.vm.step();

        self.paused = true;
    }

    pub fn continue_normal_execution(&mut self) {
        self.paused = false;
        self.screen_mode = ScreenMode::Vm;

        self.debug_enabled = false;
    }

    fn apply_debug_cmd(&mut self, cmd: &DebugCmd) {
        match cmd {
            DebugCmd::Step => {
                self.vm.step();
                self.debug_tty.clear();
                self.debug
                    .print_registers(&mut self.vm, &mut self.debug_tty);
            }
            DebugCmd::Continue => {
                self.paused = false;
                self.screen_mode = ScreenMode::Vm;
                self.debug_tty.clear();
            }
            DebugCmd::Memory => {
                self.debug_tty.clear();
                self.debug.print_memory(&mut self.vm, &mut self.debug_tty);
            }
            DebugCmd::Registers => {
                self.debug_tty.clear();
                self.debug
                    .print_registers(&mut self.vm, &mut self.debug_tty);
            }
            DebugCmd::Help => {
                self.debug_tty.clear();
                self.debug.print_help(&mut self.debug_tty);
            }
            DebugCmd::Invalid => {
                self.debug_tty.clear();
                self.debug_tty.write_byte(b'?');
                self.debug_tty.write_byte(b'\n');
                self.debug.print_help(&mut self.debug_tty);
            }
            DebugCmd::None => {}
        }
    }
}
