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

    /// Pushes a comment onto the code buffer.
    ///
    /// # Errors
    /// Returns a `CompileError` if there was an error writing to the code buffer.
    pub fn comment(&mut self, comment: &str) -> CompileResult<()> {
        writeln!(self.code, "\t; {comment}").map_err(|_| CompileError::InternalError {
            message: "Codegen error".to_string(),
        })
    }

    /// # Errors
    /// Returns a `CompileError` if there was an error writing to the code buffer.
    pub fn ldi(&mut self, reg: &str, imm: u8) -> CompileResult<()> {
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
    pub fn ld_addr(&mut self, reg: &str, addr: u16) -> CompileResult<()> {
        self.instruction(&format!("LD {reg} 0x{addr:X}"))
    }

    /// # Errors
    /// Returns a `CompileError` if there was an error writing to the code buffer.
    pub fn st_addr(&mut self, src: &str, addr: u16) -> CompileResult<()> {
        self.instruction(&format!("ST {src} 0x{addr:X}"))
    }

    /// # Errors
    /// Returns a `CompileError` if there was an error writing to the code buffer.
    pub fn add(&mut self, dst: &str, src: &str) -> CompileResult<()> {
        self.instruction(&format!("ADD {dst} {src}"))
    }

    /// # Errors
    /// Returns a `CompileError` if there was an error writing to the code buffer.
    pub fn sub(&mut self, dst: &str, src: &str) -> CompileResult<()> {
        self.instruction(&format!("SUB {dst} {src}"))
    }

    /// # Errors
    /// Returns a `CompileError` if there was an error writing to the code buffer.
    pub fn mul(&mut self, dst: &str, src: &str) -> CompileResult<()> {
        self.instruction(&format!("MUL {dst} {src}"))
    }

    /// # Errors
    /// Returns a `CompileError` if there was an error writing to the code buffer.
    pub fn div(&mut self, dst: &str, src: &str) -> CompileResult<()> {
        self.instruction(&format!("DIV {dst} {src}"))
    }

    /// # Errors
    /// Returns a `CompileError` if there was an error writing to the code buffer.
    pub fn cmp(&mut self, dst: &str, src: &str) -> CompileResult<()> {
        self.instruction(&format!("CMP {dst} {src}"))
    }

    /// # Errors
    /// Returns a `CompileError` if there was an error writing to the code buffer.
    pub fn jr(&mut self, label: &str) -> CompileResult<()> {
        self.instruction(&format!("JR {label}"))
    }

    /// # Errors
    /// Returns a `CompileError` if there was an error writing to the code buffer.
    pub fn jzr(&mut self, label: &str) -> CompileResult<()> {
        self.instruction(&format!("JZR {label}"))
    }

    /// # Errors
    /// Returns a `CompileError` if there was an error writing to the code buffer.
    pub fn jnzr(&mut self, label: &str) -> CompileResult<()> {
        self.instruction(&format!("JNZR {label}"))
    }

    /// # Errors
    /// Returns a `CompileError` if there was an error writing to the code buffer.
    pub fn jmp(&mut self, addr: &str) -> CompileResult<()> {
        self.instruction(&format!("JMP {addr}"))
    }
}
