use std::fmt::Write;

use crate::error::{CompileError, CompileResult};

#[derive(Debug)]
pub struct ProgramWriter {
    result: String,
}

impl Default for ProgramWriter {
    fn default() -> Self {
        Self {
            result: r#"#include "../asm/cpu.asm"
#include "../asm/ext.asm"

"#
            .to_string(),
        }
    }
}

impl ProgramWriter {
    #[allow(clippy::needless_pass_by_value)]
    /// Emit a single instruction line with a tab prefix.
    ///
    /// # Errors
    /// Returns `CompileError::InternalError` if writing to the buffer fails.
    pub fn emit(&mut self, value: impl ToString) -> CompileResult<()> {
        writeln!(self.result, "\t{}", value.to_string()).map_err(|_| CompileError::InternalError {
            message: "Codegen error".to_string(),
        })
    }

    #[allow(clippy::needless_pass_by_value)]
    /// Emit a top-level label.
    ///
    /// # Errors
    /// Returns `CompileError::InternalError` if writing to the buffer fails.
    pub fn label(&mut self, label: impl ToString) -> CompileResult<()> {
        writeln!(self.result, "{}:", label.to_string()).map_err(|_| CompileError::InternalError {
            message: "Codegen error".to_string(),
        })
    }

    #[allow(clippy::needless_pass_by_value)]
    /// Emit a label for a nested scope such as a basic block.
    ///
    /// # Errors
    /// Returns `CompileError::InternalError` if writing to the buffer fails.
    pub fn sublabel(&mut self, sublabel: impl ToString) -> CompileResult<()> {
        writeln!(self.result, ".{}:", sublabel.to_string()).map_err(|_| {
            CompileError::InternalError {
                message: "Codegen error".to_string(),
            }
        })
    }

    #[allow(clippy::needless_pass_by_value)]
    #[must_use]
    pub fn basic_block_label(basic_block_id: usize) -> String {
        format!("BB{basic_block_id}")
    }

    #[must_use]
    pub fn finish(&self) -> String {
        self.result.clone()
    }
}
