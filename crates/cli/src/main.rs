use std::path::PathBuf;

use clap::Parser;
use mb8::vm;
use minifb::{Window, WindowOptions};
use tty::render_tty;

mod config;
mod tty;

fn run_vm(file: PathBuf) {
    let Ok(source) = std::fs::read(file) else {
        return;
    };
    let mut vm = vm::VirtualMachine::default();
    vm.load_rom(&source);

    let Ok(mut window) = Window::new("MB8", 640, 480, WindowOptions::default()) else {
        return;
    };

    let mut buf = vec![0u32; 320 * 200];

    while !vm.halted && window.is_open() {
        vm.step();

        let tty = vm.devices.gpu().tty_buffer();
        render_tty(tty, buf.as_mut_slice());

        if window.update_with_buffer(&buf, 320, 200).is_err() {
            return;
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
