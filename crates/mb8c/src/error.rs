pub type CompileResult<T, E = CompileError> = Result<T, E>;

#[derive(Debug)]
pub enum CompileError {
    UnexpectedToken { line: usize, column: usize },
}
