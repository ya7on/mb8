use std::path::PathBuf;

use clap::Parser;
use mb8::vm;
use minifb::{Window, WindowOptions};

mod config;

fn run_vm(file: PathBuf, bot: Option<PathBuf>) {
    let Ok(source) = std::fs::read(file) else {
        return;
    };
    let mut vm = vm::VirtualMachine::default();
    vm.load_mem(&source);
    if let Some(bot) = bot {
        let Ok(source) = std::fs::read(bot) else {
            return;
        };
        vm.load_bot(&source);
    }

    let mut buf = vec![0u32; 64 * 32];

    let Ok(mut window) = Window::new("MB8", 640, 320, WindowOptions::default()) else {
        return;
    };

    let mut i = 0;
    while !vm.halted && window.is_open() {
        vm.step();

        i += 1;
        if !vm.redraw && i <= 120 {
            continue;
        }
        i = 0;

        let gfx = vm.mem.host().graphic_buffer();

        for y in 0..32 {
            for x in 0..64 {
                let index = y as usize * 64 + x as usize;
                if gfx.get_pixel(x, y) {
                    buf[index] = 0x006a_bfc6;
                } else {
                    buf[index] = 0x0050_459b;
                }
            }
        }

        if window.update_with_buffer(&buf, 64, 32).is_err() {
            return;
        }
    }
}

fn main() {
    let cli = config::Cli::parse();

    match cli.command {
        config::Commands::Run { file, bot } => {
            run_vm(file, bot);
        }
    }
}
