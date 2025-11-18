use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "mb8", version, about = "MB8 VM")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run an executable file for the VM
    Run {
        /// Path to the executable file
        file: PathBuf,
    },
}
