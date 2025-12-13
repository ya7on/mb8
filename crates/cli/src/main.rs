use ariadne::{Label, Report, ReportKind, Source};
use clap::Parser;
use mb8c::{compile, error::CompileError};

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
            let code = match std::fs::read_to_string(source.clone()) {
                Ok(code) => code,
                Err(err) => {
                    eprintln!("Failed to read source file: {err}");
                    return;
                }
            };
            match compile(&code) {
                Ok(()) => {}
                Err(errors) => {
                    let filename = source.to_str().unwrap_or("unknown");

                    for err in errors {
                        let report = match err {
                            CompileError::UnexpectedToken { start, end } => {
                                Report::build(ReportKind::Error, (filename, start..end))
                                    .with_code(1)
                                    .with_label(Label::new((filename, start..end)))
                                    .with_message("Unexpected token")
                            }
                            CompileError::ParserError {
                                start,
                                end,
                                found: Some(found),
                            } => Report::build(ReportKind::Error, (filename, start..end))
                                .with_code(1)
                                .with_label(Label::new((filename, start..end)))
                                .with_message(format!("Unexpected {found:?}")),
                            _ => Report::build(ReportKind::Error, (filename, 0..0))
                                .with_code(1)
                                .with_message("Unknown error"),
                        };

                        report
                            .finish()
                            .print((filename, Source::from(code.clone())))
                            .unwrap();
                    }
                }
            }
        }
    }
}
