use crate::{hir::types::TypeKind, lex::tokens::TokenKind};

pub type CompileResult<T, E = CompileError> = Result<T, E>;

#[derive(Debug, PartialEq, Clone, Default)]
pub enum CompileError {
    #[default]
    Unknown,
    InternalError {
        message: String,
    },
    UnexpectedToken {
        start: usize,
        end: usize,
    },
    ParserError {
        start: usize,
        end: usize,
        found: Option<TokenKind>,
    },
    UnknownSymbol {
        start: usize,
        end: usize,
        symbol: String,
    },
    DuplicateSymbol {
        start: usize,
        end: usize,
        symbol: String,
    },
    TypeMismatch {
        expected: TypeKind,
        actual: TypeKind,
        start: usize,
        end: usize,
    },
    SymbolIsNotCallable {
        symbol: String,
        start: usize,
        end: usize,
    },
    WrongArgumentsCount {
        expected: usize,
        actual: usize,
        start: usize,
        end: usize,
    },
}
