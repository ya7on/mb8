use std::{fs, path::PathBuf, process::ExitCode};

use ariadne::{sources, Color, Label, Report, ReportKind};
use asm::{compile_file, Diagnostic, DiagnosticKind, Severity, SourceFile};
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

    let mut compilation = compile_file(&cli.input);
    for diagnostic in &compilation.diagnostics {
        print_diagnostic(diagnostic, &compilation.sources);
    }

    let Some(bytes) = compilation.result.take() else {
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

fn print_diagnostic(diagnostic: &Diagnostic, source_files: &[SourceFile]) {
    let source_names: Vec<_> = source_files
        .iter()
        .map(|source| source.name.clone())
        .collect();
    let fallback_name = source_names
        .first()
        .cloned()
        .unwrap_or_else(|| "<unknown>".to_string());
    let primary = diagnostic
        .span
        .as_ref()
        .and_then(|span| {
            source_names
                .get(span.source)
                .map(|name| (name.clone(), span.range.clone()))
        })
        .unwrap_or((fallback_name, 0..0));
    let (report_kind, color) = match diagnostic.severity {
        Severity::Error => (ReportKind::Error, Color::Red),
        Severity::Warning => (ReportKind::Warning, Color::Yellow),
    };
    let mut report = Report::build(report_kind, primary)
        .with_code(diagnostic.code())
        .with_message(diagnostic.message());

    if let DiagnosticKind::DuplicateLabel { first, .. }
    | DiagnosticKind::DuplicateOrigin { first } = &diagnostic.kind
    {
        if let Some(source_name) = source_names.get(first.source) {
            report = report.with_label(
                Label::new((source_name.clone(), first.range.clone()))
                    .with_color(color)
                    .with_message("first defined here"),
            );
        }
        if let Some(span) = diagnostic.span.as_ref() {
            if let Some(source_name) = source_names.get(span.source) {
                report = report.with_label(
                    Label::new((source_name.clone(), span.range.clone()))
                        .with_color(color)
                        .with_message("duplicate here"),
                );
            }
        }
    } else if let Some(span) = diagnostic.span.as_ref() {
        if let Some(source_name) = source_names.get(span.source) {
            report = report.with_label(
                Label::new((source_name.clone(), span.range.clone()))
                    .with_color(color)
                    .with_message(diagnostic.message()),
            );
        }
    }

    let source_entries: Vec<_> = source_files
        .iter()
        .zip(source_names.iter())
        .map(|(source, name)| (name.clone(), source.source.clone()))
        .collect();

    if let Err(report_err) = report.finish().print(sources(source_entries)) {
        eprintln!("failed to print diagnostic: {report_err}");
    }
}
