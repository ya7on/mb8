use std::path::PathBuf;

use clap::Parser;
use mb8::{dev::gpu::Mode, vm};
use minifb::{Window, WindowOptions};

mod config;

fn run_vm(file: PathBuf) {
    let Ok(source) = std::fs::read(file) else {
        return;
    };
    let mut vm = vm::VirtualMachine::default();
    vm.load_rom(&source);

    let Ok(mut window) = Window::new("MB8", 640, 320, WindowOptions::default()) else {
        return;
    };

    while !vm.halted && window.is_open() {
        vm.step();

        if vm.devices.gpu.mode == Mode::Tty {
            let mut buffer = vec![0; 128 * 64 * 4];
            for (i, sym) in vm.devices.gpu.tty.buffer.into_iter().enumerate() {
                let (x, y) = (i % 640, i / 640);
                buffer[(x + y * 640) * 4..(x + y * 640) * 4 + 4].copy_from_slice(todo!());
            }
            window.update_with_buffer(&buffer, 640, 320).unwrap();
        }
    }
}

fn main() {
    let cli = config::Cli::parse();

    match cli.command {
        config::Commands::Run { file } => {
            run_vm(file);
        }
    }
}
