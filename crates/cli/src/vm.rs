use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

use crate::{filesystem::makefs, keyboard::Keyboard};
use mb8::{
    dev::gpu::registers::{TTY_COLS, TTY_ROWS},
    vm,
};

use minifb::{Window, WindowOptions};

use crate::tty::Tty;

const OPS_PER_FRAME: u32 = 1024;
const RENDER_INTERVAL: u32 = 1000;
#[allow(clippy::too_many_lines)]
pub fn run_vm(kernel: PathBuf, user: Vec<PathBuf>, seed: Option<u16>) {
    let Ok(rom) = std::fs::read(kernel) else {
        return;
    };
    let mut vm = vm::VirtualMachine::default();
    vm.load_rom(&rom);

    let Ok(mut window) = Window::new("MB8", 640, 480, WindowOptions::default()) else {
        return;
    };

    let seed = seed.unwrap_or(1);

    let mut tty = Tty::new(TTY_COLS as usize, TTY_ROWS as usize, 1024);
    vm.devices.rand().number = (seed as u8).max(1);

    makefs(user, &mut vm);

    let mut buf = vec![0u32; 320 * 200];
    let mut ticks = RENDER_INTERVAL - 1;
    let mut last_render = Instant::now();
    let l_shift = false;
    let r_shift = false;
    let key = &mut Keyboard::new(l_shift, r_shift);

    while !vm.halted && window.is_open() {
        ticks = ticks.wrapping_add(1);

        Keyboard::key_pressed(key, &window, &mut vm);

        Keyboard::key_released(key, &window);

        for _ in 0..OPS_PER_FRAME {
            if vm.halted {
                break;
            }
            vm_step(&mut vm);
        }

        if last_render.elapsed() >= Duration::from_millis(16) {
            let gpu = vm.devices.gpu();
            for &byte in gpu.tty_buffer() {
                tty.write_byte(byte);
            }

            tty.render(buf.as_mut_slice(), 320);

            if window.update_with_buffer(&buf, 320, 200).is_err() {
                return;
            }
            last_render = Instant::now();
        }
    }
}

fn vm_step(vm: &mut vm::VirtualMachine) {
    vm.step();
}
