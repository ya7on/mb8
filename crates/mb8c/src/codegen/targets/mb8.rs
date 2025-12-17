use std::fmt::Write;

use crate::{
    error::{CompileError, CompileResult},
    ir::{IRFunction, IRInstruction, IRProgram},
};

#[derive(Debug, Default)]
pub struct Mb8Codegen {
    result: String,
}

impl Mb8Codegen {
    fn emit(&mut self, value: &str) -> CompileResult<()> {
        writeln!(self.result, "\t{value}").map_err(|_| CompileError::InternalError {
            message: "Codegen error".to_string(),
        })
    }

    fn label(&mut self, label: &str) -> CompileResult<()> {
        writeln!(self.result, "{label}:").map_err(|_| CompileError::InternalError {
            message: "Codegen error".to_string(),
        })
    }

    fn sublabel(&mut self, sublabel: &str) -> CompileResult<()> {
        writeln!(self.result, ".{sublabel}:").map_err(|_| CompileError::InternalError {
            message: "Codegen error".to_string(),
        })
    }

    fn basic_block_label(function_name: &str, basic_block_id: usize) -> String {
        format!("{function_name}_{basic_block_id}")
    }

    /// # Errors
    /// Returns error if there are codegen issues
    pub fn codegen(&mut self, ir: &IRProgram) -> CompileResult<String> {
        for function in &ir.functions {
            self.codegen_function(function)?;
        }

        Ok(self.result.clone())
    }

    fn codegen_function(&mut self, function: &IRFunction) -> CompileResult<()> {
        self.label(&function.name)?;

        for basic_block in &function.basic_blocks {
            self.sublabel(&Self::basic_block_label(&function.name, basic_block.id.0))?;

            for instruction in &basic_block.instructions {
                self.codegen_instruction(instruction)?;
            }
        }

        Ok(())
    }

    fn codegen_instruction(&mut self, instruction: &IRInstruction) -> CompileResult<()> {
        match instruction {
            //     IRInstruction::LoadImm { register, value } => {
            //         self.emit("LDA R0 0x100500")?;
            //     }
            //     IRInstruction::Store { register, offset } => {
            //         self.emit("ST [IH:IL] R0")?;
            //     }
            //     IRInstruction::Load { register, offset } => {
            //         self.emit("LD R0 [IH:IL]")?;
            //     }
            //     IRInstruction::Add { dst, lhs, rhs } => {
            //         self.emit("ADD R1 R0")?;
            //     }
            //     IRInstruction::Sub { dst, lhs, rhs } => {
            //         self.emit("SUB R1 R0")?;
            //     }
            //     IRInstruction::Mul { dst, lhs, rhs } => {
            //         self.emit("MUL R1 R0")?;
            //     }
            //     IRInstruction::Div { dst, lhs, rhs } => {
            //         self.emit("DIV R1 R0")?;
            //     }
            //     IRInstruction::Cmp { dst, lhs, rhs } => {
            //         self.emit("CMP R1 R0")?;
            //     }
            //     IRInstruction::Neg { dst, src } => {
            //         self.emit("NEG R0")?;
            //     }
            IRInstruction::Call {
                result: _,
                label,
                args: _,
            } => {
                self.emit(&format!("CALL [{label}]"))?;
            }
            _ => todo!(),
        }

        Ok(())
    }
}
