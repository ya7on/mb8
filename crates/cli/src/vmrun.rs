use crate::{filesystem::makefs, keyboard::Keyboard};
use std::path::PathBuf;

use mb8::vm;
use minifb::{Window, WindowOptions};

use crate::debug::{Debug, DebugCmd};
use crate::tty::Tty;

use std::io::{self, Write};
use std::time::{Duration, Instant};

const OPS_PER_FRAME: u32 = 1024;
const RENDER_INTERVAL: u32 = 1000;
const WIDTH: usize = 320;
const HEIGHT: usize = 200;
const FRAME_DURATION_MS: u64 = 16;

#[derive(Debug)]
pub struct VmRun {
    pub vm: vm::VirtualMachine,
    pub tty: Tty,
    pub window: Window,
    ticks: u32,
    debug: Debug,
    debug_input: Vec<u8>,
    pub debug_enabled: bool,
    pub hit_entry_break: bool,
    pub paused: bool,
}

impl VmRun {
    /// Create a bew VM Runtime
    ///
    /// # Errors
    ///
    /// Returns `Err(minifb::Error)` if the window or framebuffer
    /// cannot be initialized.
    pub fn new(vm: vm::VirtualMachine, tty: Tty, debug: Debug) -> Result<Self, minifb::Error> {
        let window = Window::new("MB8", 640, 480, WindowOptions::default())?;
        Ok(Self {
            vm,
            tty,
            window,
            ticks: 0,
            debug,
            debug_input: Vec::new(),
            debug_enabled: false,
            hit_entry_break: false,
            paused: false,
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

        //pre allocate the buffer.. maybe that will help.
        let mut buf = vec![0u32; WIDTH * HEIGHT];

        let mut last_frame = std::time::Instant::now();

        while self.window.is_open() && !self.vm.halted {
            Keyboard::key_pressed(key, &self.window, &mut self.vm);
            Keyboard::key_released(key, &self.window);
            if self.debug_enabled {
                // Step VM once to finish printing any pending TTY output
                self.vm_step();

                // Enter debug mode
                let paused = self.run_debug();
                if paused {
                    self.poll_debug_keys_stdout();
                    continue; // skip rest of loop while paused
                }
            } else {
                self.vm_step();
            }

            if last_frame.elapsed() >= Duration::from_millis(FRAME_DURATION_MS) {
                self.render(&mut buf);
                last_frame = Instant::now();
            }
        }

        self.render(&mut buf);
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

        // Break at user entry exactly once
        if self.debug_enabled && !self.hit_entry_break && self.vm.program_counter == USER_ENTRY {
            self.hit_entry_break = true;
            self.paused = true;

            // Clear terminal + show debugger help once
            print!("\x1B[2J\x1B[H");
            println!("--- Debugger Break @ {USER_ENTRY:04X} ---");
            self.debug.print_help();

            self.run_stdout_debugger();
        }

        self.paused
    }

    fn render(&mut self, buf: &mut [u32]) {
        let gpu_tty = self.vm.devices.gpu().tty_buffer();
        self.tty.load_from_slice(gpu_tty);

        if gpu_tty.iter().all(|&b| b == 0) {
            println!("GPU TTY BUFFER IS EMPTY");
        }

        self.tty.render(buf, WIDTH);

        // Update window and handle errors
        if let Err(e) = self.window.update_with_buffer(buf, WIDTH, HEIGHT) {
            eprintln!("Failed to update window: {e:?}");
        }
    }

    fn poll_debug_keys_stdout(&mut self) {
        for key in self.window.get_keys_pressed(minifb::KeyRepeat::No) {
            if let Some(byte) = Debug::map_debug_key(key) {
                if let Some(cmd) = self.debug.handle_debug_byte(byte, &mut self.debug_input) {
                    self.apply_debug_cmd_stdout(&cmd);
                    self.debug_input.clear();

                    println!();
                    print!("> ");
                    let _ = io::stdout().flush();
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

        self.debug_enabled = false;
    }

    fn apply_debug_cmd_stdout(&mut self, cmd: &DebugCmd) {
        match cmd {
            DebugCmd::Step => {
                self.vm.step();

                print!("\x1B[2J\x1B[H");
                let _ = io::stdout().flush();

                self.debug.print_registers(&mut self.vm);
            }

            DebugCmd::Continue => {
                self.paused = false;

                print!("\x1B[2J\x1B[H");
                let _ = io::stdout().flush();
            }

            DebugCmd::Memory(arg) => {
                print!("\x1B[2J\x1B[H");
                let _ = io::stdout().flush();

                if let Some(addr_str) = arg {
                    self.debug
                        .parse_and_print_memory_stdout(addr_str, &mut self.vm);
                } else {
                    self.debug
                        .print_memory_range_stdout(&mut self.vm, 0x0000, 0x00FF);
                }
            }

            DebugCmd::Registers => {
                print!("\x1B[2J\x1B[H");
                let _ = io::stdout().flush();

                self.debug.print_registers(&mut self.vm);
            }

            DebugCmd::Help => {
                print!("\x1B[2J\x1B[H");
                let _ = io::stdout().flush();

                self.debug.print_help();
            }

            DebugCmd::Invalid => {
                print!("\x1B[2J\x1B[H");
                println!("?");
                self.debug.print_help();
                let _ = io::stdout().flush();
            }

            DebugCmd::None => {}
        }
    }

    fn run_debug_repl(&mut self) {
        let mut input = String::new();

        loop {
            print!("> ");
            let _ = io::stdout().flush();

            input.clear();
            if io::stdin().read_line(&mut input).is_err() {
                println!("Failed to read input");
                continue;
            }

            let input_trimmed = input.trim();
            if input_trimmed.is_empty() {
                continue;
            }

            let mut bytes = input_trimmed.bytes().collect::<Vec<u8>>();
            bytes.push(b'\n');

            let mut debug_input: Vec<u8> = Vec::new();
            let mut cmd_opt: Option<DebugCmd> = None;

            for b in bytes {
                if let Some(cmd) = self.debug.handle_debug_byte(b, &mut debug_input) {
                    cmd_opt = Some(cmd);
                }
            }

            if let Some(cmd) = cmd_opt {
                self.apply_debug_cmd_stdout(&cmd);
            }

            if !self.paused {
                break;
            }
        }
    }

    pub fn run_stdout_debugger(&mut self) {
        self.paused = true;
        self.debug_enabled = true;

        while !self.vm.halted {
            if self.paused {
                self.run_debug_repl();
            }

            if !self.paused {
                self.vm_step();
                if !self.paused {
                    break;
                }
            } else {
                break;
            }
        }

        println!("--- VM Halted ---");
    }
}
