use std::fmt::Write;

use crate::error::{CompileError, CompileResult};

#[derive(Debug, Default)]
pub struct Mb8Asm {
    pub code: String,
}

impl Mb8Asm {
    /// Pushes a label onto the code buffer.
    ///
    /// # Errors
    /// Returns a `CompileError` if there was an error writing to the code buffer.
    pub fn label(&mut self, label: &str) -> CompileResult<()> {
        writeln!(self.code, "{label}:").map_err(|_| CompileError::InternalError {
            message: "Codegen error".to_string(),
        })
    }

    /// Pushes an instruction onto the code buffer.
    ///
    /// # Errors
    /// Returns a `CompileError` if there was an error writing to the code buffer.
    pub fn instruction(&mut self, instruction: &str) -> CompileResult<()> {
        writeln!(self.code, "\t{instruction}").map_err(|_| CompileError::InternalError {
            message: "Codegen error".to_string(),
        })
    }

    /// # Errors
    /// Returns a `CompileError` if there was an error writing to the code buffer.
    pub fn ldi(&mut self, reg: &str, imm: i64) -> CompileResult<()> {
        self.instruction(&format!("LDI {reg} {imm}"))
    }

    /// # Errors
    /// Returns a `CompileError` if there was an error writing to the code buffer.
    pub fn ret(&mut self) -> CompileResult<()> {
        self.instruction("RET")
    }

    /// # Errors
    /// Returns a `CompileError` if there was an error writing to the code buffer.
    pub fn call(&mut self, name: &str) -> CompileResult<()> {
        self.instruction(&format!("CALL {name}"))
    }

    /// # Errors
    /// Returns a `CompileError` if there was an error writing to the code buffer.
    pub fn st16(&mut self, reg: &str, offset: i16) -> CompileResult<()> {
        self.instruction(&format!("ST {reg} 0x{offset:X}"))
    }
}
