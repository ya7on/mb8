use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

use crate::{filesystem::makefs, keyboard::Keyboard};
use mb8::vm;
use minifb::{Window, WindowOptions};

use crate::tty::Tty;

const OPS_PER_FRAME: u32 = 1024;
const RENDER_INTERVAL: u32 = 1000;

#[derive(Debug)]
pub struct VmRun {
    pub vm: vm::VirtualMachine,
    pub tty: Tty,
    ticks: u32,
    width: usize,
    height: usize,
}

impl VmRun {
    #[must_use]
    pub fn new(vm: vm::VirtualMachine, tty: Tty) -> Self {
        Self {
            vm,
            tty,
            ticks: 0,
            width: 320,
            height: 200,
        }
    }

    pub fn run_desktop(&mut self, kernel: PathBuf, user: Vec<PathBuf>, seed: Option<u16>) {
        let Ok(rom) = std::fs::read(kernel) else {
            return;
        };
        self.vm.load_rom(&rom);

        let Ok(mut window) = Window::new("MB8", 640, 480, WindowOptions::default()) else {
            return;
        };

        let seed = seed.unwrap_or(1);

        self.vm.devices.rand().number = (seed as u8).max(1);

        makefs(user, &mut self.vm);

        let mut buf = vec![0u32; self.width * self.height];
        self.ticks = RENDER_INTERVAL - 1;
        let mut last_render = Instant::now();
        let l_shift = false;
        let r_shift = false;
        let key = &mut Keyboard::new(l_shift, r_shift);

        while !self.vm.halted && window.is_open() {
            self.ticks = self.ticks.wrapping_add(1);

            Keyboard::key_pressed(key, &window, &mut self.vm);

            Keyboard::key_released(key, &window);

            self.vm_step();

            if last_render.elapsed() >= Duration::from_millis(16) {
                let gpu = self.vm.devices.gpu();
                for &byte in gpu.tty_buffer() {
                    self.tty.write_byte(byte);
                }

                self.tty.render(buf.as_mut_slice(), 320);

                if window.update_with_buffer(&buf, 320, 200).is_err() {
                    return;
                }
                last_render = Instant::now();
            }
        }
    }

    fn vm_step(&mut self) {
        for _ in 0..OPS_PER_FRAME {
            if self.vm.halted {
                break;
            }

            self.vm.step();
        }
    }
}
