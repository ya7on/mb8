use crate::tokens::TokenKind;

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
}
