use std::{fs, path::PathBuf, process::ExitCode};

use ariadne::{sources, Color, Label, Report, ReportKind};
use asm::{compile_file, AsmErrorKind, AsmFailure};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "asm", version, about = "MB8 assembler")]
struct Cli {
    input: PathBuf,

    #[arg(short, long)]
    output: PathBuf,

    #[arg(long)]
    dump: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let bytes = compile_file(&cli.input).map_err(|failure| {
        print_diagnostic(&failure);
    });
    let Ok(bytes) = bytes else {
        return ExitCode::FAILURE;
    };

    if cli.dump {
        for (address, chunk) in bytes.chunks(16).enumerate() {
            print!("{:04x}:", address * 16);
            for byte in chunk {
                print!(" {byte:02x}");
            }
            println!();
        }
    }

    let result = fs::write(&cli.output, bytes).map_err(|err| {
        eprintln!("failed to write {}: {err}", cli.output.display());
    });
    if result.is_err() {
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn print_diagnostic(failure: &AsmFailure) {
    let source_names: Vec<_> = failure
        .sources
        .iter()
        .map(|source| source.name.clone())
        .collect();
    let primary = failure.error.span().map_or_else(
        || (source_names[0].clone(), 0..0),
        |span| (source_names[span.source].clone(), span.range.clone()),
    );
    let mut report = Report::build(ReportKind::Error, primary)
        .with_code(failure.error.code())
        .with_message(failure.error.message());

    if let AsmErrorKind::DuplicateLabel { first, .. } | AsmErrorKind::DuplicateOrigin { first } =
        &failure.error.kind
    {
        report = report.with_label(
            Label::new((source_names[first.source].clone(), first.range.clone()))
                .with_color(Color::Red)
                .with_message("first defined here"),
        );
        if let Some(span) = failure.error.span() {
            report = report.with_label(
                Label::new((source_names[span.source].clone(), span.range.clone()))
                    .with_color(Color::Red)
                    .with_message("duplicate here"),
            );
        }
    } else if let Some(span) = failure.error.span() {
        report = report.with_label(
            Label::new((source_names[span.source].clone(), span.range.clone()))
                .with_color(Color::Red)
                .with_message(failure.error.message()),
        );
    }

    let source_entries: Vec<_> = failure
        .sources
        .iter()
        .zip(source_names.iter())
        .map(|(source, name)| (name.clone(), source.source.clone()))
        .collect();

    if let Err(report_err) = report.finish().print(sources(source_entries)) {
        eprintln!("failed to print diagnostic: {report_err}");
    }
}
