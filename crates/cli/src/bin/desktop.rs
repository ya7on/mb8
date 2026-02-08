use clap::Parser;
use mb8::{
    dev::gpu::registers::{BITMAP_HEIGHT, BITMAP_WIDTH, TTY_COLS, TTY_ROWS},
    vm,
};
use mb8_cli::{bitmap::Bitmap, config, debug::Debug};
use mb8_cli::{tty::Tty, vmrun};
use mb8c::compile;

fn main() {
    let cli = config::Cli::parse();

    match cli.command {
        config::Commands::Run {
            kernel,
            user,
            debug,
        } => {
            let vm = vm::VirtualMachine::default();
            let tty = Tty::new(TTY_COLS as usize, TTY_ROWS as usize, 1024);
            let bitmap = Bitmap::new(BITMAP_WIDTH, BITMAP_HEIGHT);
            let debugcli = Debug::new();
            let mut vm_desk = match vmrun::VmRun::new(vm, tty, bitmap, debugcli) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Failed to init VM: {e}");
                    return;
                }
            };
            if debug {
                vm_desk.debug_enabled = true;
            }
            vm_desk.run_desktop(kernel, user, cli.seed);
        }
        config::Commands::Compile { source } => {
            let code = match std::fs::read_to_string(source) {
                Ok(code) => code,
                Err(err) => {
                    eprintln!("Failed to read source file: {err}");
                    return;
                }
            };
            match compile(&code) {
                Ok(()) => {}
                Err(err) => {
                    eprintln!("Compilation error: {err:?}");
                }
            }
        }
    }
}
