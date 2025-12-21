use clap::Parser;
use compile::run_compiler;

mod compile;
mod config;
mod filesystem;
mod keyboard;
mod tty;
mod vm;

#[allow(clippy::too_many_lines)]
fn main() {
    let cli = config::Cli::parse();

    match cli.command {
        config::Commands::Run { kernel, user } => {
            vm::run_vm(kernel, user, cli.seed);
        }
        config::Commands::Compile { source } => run_compiler(&source),
    }
}
