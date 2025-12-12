use crate::ast::Type;

pub type CompileResult<T, E = CompileError> = Result<T, E>;

#[derive(Debug, PartialEq)]
pub enum CompileError {
    InternalError {
        message: String,
    },
    UnexpectedToken {
        line: usize,
        column: usize,
    },
    ParseError {
        line: usize,
        column: usize,
        message: String,
    },
    DuplicateFunction {
        name: String,
    },
    DuplicateVariable {
        name: String,
    },
    TypeMismatch {
        expected: Type,
        found: Type,
    },
    UndefinedSymbol {
        name: String,
    },
    InvalidArgumentCount {
        expected: usize,
        found: usize,
    },
}
