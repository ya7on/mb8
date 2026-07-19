use std::fmt;

use crate::ast::{Directive, Operand};
use crate::diagnostics::{SourceFile, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsmError {
    pub span: Option<Span>,
    pub kind: AsmErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AsmErrorKind {
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
}

impl AsmError {
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self.kind {
            AsmErrorKind::Lex { .. } => "ASM0100",
            AsmErrorKind::Parse { .. } => "ASM0101",
            AsmErrorKind::Include { .. } => "ASM0200",
            AsmErrorKind::UnsupportedInstruction { .. } => "ASM0300",
            AsmErrorKind::DuplicateLabel { .. } => "ASM0301",
            AsmErrorKind::UnknownLabel { .. } => "ASM0302",
            AsmErrorKind::UnexpectedDirective { .. } => "ASM0303",
            AsmErrorKind::DuplicateOrigin { .. } => "ASM0304",
            AsmErrorKind::AddressOverflow { .. } => "ASM0305",
            AsmErrorKind::ValueOutOfRange { .. } => "ASM0306",
            AsmErrorKind::RelativeJumpOutOfRange { .. } => "ASM0307",
        }
    }

    #[must_use]
    pub fn message(&self) -> String {
        match &self.kind {
            AsmErrorKind::Lex { message } => format!("Lex error: {message}"),
            AsmErrorKind::Parse { message } => format!("Parse error: {message}"),
            AsmErrorKind::Include { message } => format!("Include error: {message}"),
            AsmErrorKind::UnsupportedInstruction { mnemonic, operands } => {
                format!("Unsupported instruction form: {mnemonic} {operands:?}")
            }
            AsmErrorKind::DuplicateLabel { label, .. } => format!("Duplicate label: {label}"),
            AsmErrorKind::UnknownLabel { label } => format!("Unknown label: {label}"),
            AsmErrorKind::UnexpectedDirective { directive } => {
                format!("Unexpected directive after include expansion: {directive:?}")
            }
            AsmErrorKind::DuplicateOrigin { .. } => "Duplicate origin directive".to_string(),
            AsmErrorKind::AddressOverflow { current } => {
                format!("Address overflow at 0x{current:04x}")
            }
            AsmErrorKind::ValueOutOfRange { value, expected } => {
                format!("Value 0x{value:04x} is out of range for {expected}")
            }
            AsmErrorKind::RelativeJumpOutOfRange { offset } => {
                format!("Relative jump offset {offset} is out of range for i8")
            }
        }
    }

    #[must_use]
    pub const fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl fmt::Display for AsmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl std::error::Error for AsmError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsmFailure {
    pub error: AsmError,
    pub sources: Vec<SourceFile>,
}

impl fmt::Display for AsmFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl std::error::Error for AsmFailure {}
