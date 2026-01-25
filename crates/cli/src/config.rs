use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "mb8", version, about = "MB8 VM")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    pub seed: Option<u16>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run an executable file for the VM
    Run {
        /// Path to the executable file
        kernel: PathBuf,

        /// Path to the user spaace
        user: Vec<PathBuf>,

        /// debug variable
        #[arg(long)]
        debug: bool,
    },
    /// Compile a source file to an executable file
    Compile {
        /// Path to the source file
        source: PathBuf,
    },
}
