use std::{fmt, ops::Range};

use mb8_isa::registers::Register;

use crate::ast::{Directive, Operand};

pub type SourceId = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub source: SourceId,
    pub range: Range<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spanned<T> {
    pub value: T,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceFile {
    pub id: SourceId,
    pub name: String,
    pub source: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub severity: Severity,
    pub span: Option<Span>,
    pub kind: DiagnosticKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagnosticKind {
    Lex {
        message: String,
    },
    Parse {
        message: String,
    },
    Include {
        message: String,
    },
    UnsupportedInstruction {
        mnemonic: String,
        operands: Vec<Operand>,
    },
    DuplicateLabel {
        label: String,
        first: Span,
    },
    UnknownLabel {
        label: String,
    },
    UnexpectedDirective {
        directive: Directive,
    },
    DuplicateOrigin {
        first: Span,
    },
    InvalidAddressDirective {
        current: u16,
        target: u16,
    },
    AddressOverflow {
        current: u16,
    },
    ValueOutOfRange {
        value: u16,
        expected: &'static str,
    },
    RelativeJumpOutOfRange {
        offset: i32,
    },
    ScratchRegisterConflict {
        mnemonic: String,
        register: Register,
    },
    DuplicateConstant {
        name: String,
        first: Span,
    },
    UnknownConstant {
        name: String,
    },
}

impl Diagnostic {
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self.kind {
            DiagnosticKind::Lex { .. } => "A0100",
            DiagnosticKind::Parse { .. } => "A0101",
            DiagnosticKind::Include { .. } => "A0200",
            DiagnosticKind::UnsupportedInstruction { .. } => "A0300",
            DiagnosticKind::DuplicateLabel { .. } => "A0301",
            DiagnosticKind::UnknownLabel { .. } => "A0302",
            DiagnosticKind::UnexpectedDirective { .. } => "A0303",
            DiagnosticKind::DuplicateOrigin { .. } => "A0304",
            DiagnosticKind::AddressOverflow { .. } => "A0305",
            DiagnosticKind::ValueOutOfRange { .. } => "A0306",
            DiagnosticKind::RelativeJumpOutOfRange { .. } => "A0307",
            DiagnosticKind::ScratchRegisterConflict { .. } => "A0308",
            DiagnosticKind::InvalidAddressDirective { .. } => "A0309",
            DiagnosticKind::DuplicateConstant { .. } => "A0310",
            DiagnosticKind::UnknownConstant { .. } => "A0311",
        }
    }

    #[must_use]
    pub fn message(&self) -> String {
        match &self.kind {
            DiagnosticKind::Lex { message } => format!("Lex error: {message}"),
            DiagnosticKind::Parse { message } => format!("Parse error: {message}"),
            DiagnosticKind::Include { message } => format!("Include error: {message}"),
            DiagnosticKind::UnsupportedInstruction { mnemonic, operands } => {
                let operands = operands
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                if operands.is_empty() {
                    format!("Unsupported instruction form: {mnemonic}")
                } else {
                    format!("Unsupported instruction form: {mnemonic} {operands}")
                }
            }
            DiagnosticKind::DuplicateLabel { label, .. } => {
                format!("Duplicate label: {label}")
            }
            DiagnosticKind::UnknownLabel { label } => format!("Unknown label: {label}"),
            DiagnosticKind::UnexpectedDirective { directive } => {
                format!("Unexpected directive after include expansion: {directive:?}")
            }
            DiagnosticKind::DuplicateOrigin { .. } => "Duplicate origin directive".to_string(),
            DiagnosticKind::InvalidAddressDirective { current, target } => {
                format!(
                    "Address directive targets 0x{target:04x}, but the current address is 0x{current:04x}"
                )
            }
            DiagnosticKind::AddressOverflow { current } => {
                format!("Address overflow at 0x{current:04x}")
            }
            DiagnosticKind::ValueOutOfRange { value, expected } => {
                format!("Value 0x{value:04x} is out of range for {expected}")
            }
            DiagnosticKind::RelativeJumpOutOfRange { offset } => {
                format!("Relative jump offset {offset} is out of range for i8")
            }
            DiagnosticKind::ScratchRegisterConflict { mnemonic, register } => {
                format!("Scratch register {register:?} conflicts with an operand of {mnemonic}")
            }
            DiagnosticKind::DuplicateConstant { name, .. } => {
                format!("Duplicate constant: @{name}")
            }
            DiagnosticKind::UnknownConstant { name } => format!("Unknown constant: @{name}"),
        }
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

#[derive(Debug)]
pub struct DiagnosticResult<T> {
    pub result: Option<T>,
    pub diagnostics: Vec<Diagnostic>,
    pub ok: bool,
    pub sources: Vec<SourceFile>,
}

impl<T> DiagnosticResult<T> {
    #[must_use]
    pub fn finalize(mut self) -> Self {
        if self
            .diagnostics
            .iter()
            .any(|diagnostic| matches!(diagnostic.severity, Severity::Error))
        {
            self.ok = false;
            self.result = None;
        }
        self
    }
}
