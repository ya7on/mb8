use std::{
    fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

use ariadne::{sources, Color, Label, Report, ReportKind};
use asm::{compile_file, Diagnostic, DiagnosticKind, DiagnosticResult, Severity, SourceFile};
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "asm", version, about = "MB8 assembler")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Compile an assembly source and write its binary image.
    Build(BuildArgs),
    /// Compile an assembly source without writing an output file.
    Check(CheckArgs),
}

#[derive(Debug, Args)]
struct BuildArgs {
    input: PathBuf,

    #[arg(short, long)]
    output: PathBuf,

    #[arg(long)]
    dump: bool,
}

#[derive(Debug, Args)]
struct CheckArgs {
    input: PathBuf,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match &cli.command {
        Command::Build(args) => build(args),
        Command::Check(args) => check(args),
    }
}

fn compile_and_report(input: &Path) -> DiagnosticResult<Vec<u8>> {
    let compilation = compile_file(input);
    for diagnostic in &compilation.diagnostics {
        print_diagnostic(diagnostic, &compilation.sources);
    }
    compilation
}

fn build(args: &BuildArgs) -> ExitCode {
    let mut compilation = compile_and_report(&args.input);
    if compilation
        .diagnostics
        .iter()
        .any(|diagnostic| matches!(diagnostic.severity, Severity::Error))
    {
        return ExitCode::FAILURE;
    }
    let Some(bytes) = compilation.result.take() else {
        return ExitCode::FAILURE;
    };

    if args.dump {
        for (address, chunk) in bytes.chunks(16).enumerate() {
            print!("{:04x}:", address * 16);
            for byte in chunk {
                print!(" {byte:02x}");
            }
            println!();
        }
    }

    let result = fs::write(&args.output, bytes).map_err(|err| {
        eprintln!("failed to write {}: {err}", args.output.display());
    });
    if result.is_err() {
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn check(args: &CheckArgs) -> ExitCode {
    let compilation = compile_and_report(&args.input);
    if compilation.result.is_none()
        || compilation
            .diagnostics
            .iter()
            .any(|diagnostic| matches!(diagnostic.severity, Severity::Error | Severity::Warning))
    {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
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
        .with_message(diagnostic.message())
        .with_note(format!(
            "For more information\nhttps://ya7on.github.io/mb8/asm/diagnostics.html#{}",
            diagnostic.code().to_ascii_lowercase()
        ));

    if let DiagnosticKind::UnsupportedInstruction { mnemonic, .. } = &diagnostic.kind {
        report = report.with_note(format!(
            "For mnemonic documentation\nhttps://ya7on.github.io/mb8/asm/instructions.html#{}",
            mnemonic.to_ascii_lowercase()
        ));
    }

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
