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
        token: String,
    },
}
