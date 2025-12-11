use clap::Parser;
use mb8c::compile;

mod config;
mod filesystem;
mod keyboard;
mod tty;
mod vm;

fn main() {
    let cli = config::Cli::parse();

    match cli.command {
        config::Commands::Run { kernel, user } => {
            vm::run_vm(kernel, user, cli.seed);
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
