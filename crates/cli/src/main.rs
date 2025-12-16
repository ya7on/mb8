use ariadne::{Color, Label, Report, ReportKind, Source};
use clap::Parser;
use mb8c::{compile, error::CompileError};

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
        config::Commands::Compile { source } => {
            let code = match std::fs::read_to_string(source.clone()) {
                Ok(code) => code,
                Err(err) => {
                    eprintln!("Failed to read source file: {err}");
                    return;
                }
            };
            match compile(&code) {
                Ok(()) => {
                    println!("Successfuly compiled");
                }
                Err(errors) => {
                    let filename = source.to_str().unwrap_or("unknown");

                    for err in errors {
                        let report = match err {
                            CompileError::UnexpectedToken { start, end } => {
                                Report::build(ReportKind::Error, (filename, start..end))
                                    .with_code(1)
                                    .with_label(
                                        Label::new((filename, start..end)).with_color(Color::Red),
                                    )
                                    .with_message("Unexpected token")
                            }
                            CompileError::UnknownSymbol { start, end, symbol } => {
                                Report::build(ReportKind::Error, (filename, start..end))
                                    .with_code(2)
                                    .with_label(
                                        Label::new((filename, start..end)).with_color(Color::Red),
                                    )
                                    .with_message(format!("Unknown symbol '{symbol}'"))
                            }
                            CompileError::DuplicateSymbol { start, end, symbol } => {
                                Report::build(ReportKind::Error, (filename, start..end))
                                    .with_code(3)
                                    .with_label(
                                        Label::new((filename, start..end)).with_color(Color::Red),
                                    )
                                    .with_message(format!("Duplicate symbol '{symbol}'"))
                            }
                            CompileError::TypeMismatch {
                                expected,
                                actual,
                                start,
                                end,
                            } => Report::build(ReportKind::Error, (filename, start..end))
                                .with_code(4)
                                .with_label(
                                    Label::new((filename, start..end)).with_color(Color::Red),
                                )
                                .with_message(format!(
                                    "Type mismatch: expected {expected:?}, found {actual:?}"
                                )),
                            CompileError::SymbolIsNotCallable { symbol, start, end } => {
                                Report::build(ReportKind::Error, (filename, start..end))
                                    .with_code(5)
                                    .with_label(
                                        Label::new((filename, start..end)).with_color(Color::Red),
                                    )
                                    .with_message(format!("Symbol {symbol} is not callable"))
                            }
                            CompileError::ParserError {
                                start,
                                end,
                                found: Some(found),
                            } => Report::build(ReportKind::Error, (filename, start..end))
                                .with_code(6)
                                .with_label(
                                    Label::new((filename, start..end)).with_color(Color::Red),
                                )
                                .with_message(format!("Unexpected {found:?}")),
                            CompileError::WrongArgumentsCount {
                                actual,
                                expected,
                                start,
                                end,
                            } => Report::build(ReportKind::Error, (filename, start..end))
                                .with_code(7)
                                .with_label(
                                    Label::new((filename, start..end)).with_color(Color::Red),
                                )
                                .with_message(format!(
                                    "Wrong argument count: expected {expected} found {actual}"
                                )),
                            err => Report::build(ReportKind::Error, (filename, 0..0))
                                .with_code(0)
                                .with_message(format!("Unknown error: {err:?}")),
                        };

                        #[allow(clippy::unwrap_used)]
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
