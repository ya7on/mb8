use std::path::PathBuf;

use clap::Parser;
use mb8::vm;
use minifb::{Window, WindowOptions};

mod config;

fn run_vm(file: PathBuf) {
    let Ok(source) = std::fs::read(file) else {
        return;
    };
    let mut vm = vm::VirtualMachine::default();
    vm.load_rom(&source);

    let Ok(window) = Window::new("MB8", 640, 320, WindowOptions::default()) else {
        return;
    };

    while !vm.halted && window.is_open() {
        vm.step();
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
