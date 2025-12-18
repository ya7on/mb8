use std::fmt::Write;

use crate::{
    error::{CompileError, CompileResult},
    ir::{BasicBlockTerminator, IRFunction, IRInstruction, IRProgram, VirtualRegister},
};

#[derive(Debug, Default)]
pub struct Mb8Codegen {
    result: String,
}

impl Mb8Codegen {
    fn emit(&mut self, value: impl ToString) -> CompileResult<()> {
        writeln!(self.result, "\t{}", value.to_string()).map_err(|_| CompileError::InternalError {
            message: "Codegen error".to_string(),
        })
    }

    fn label(&mut self, label: impl ToString) -> CompileResult<()> {
        writeln!(self.result, "{}:", label.to_string()).map_err(|_| CompileError::InternalError {
            message: "Codegen error".to_string(),
        })
    }

    fn sublabel(&mut self, sublabel: impl ToString) -> CompileResult<()> {
        writeln!(self.result, ".{}:", sublabel.to_string()).map_err(|_| {
            CompileError::InternalError {
                message: "Codegen error".to_string(),
            }
        })
    }

    fn basic_block_label(function_name: impl ToString, basic_block_id: impl ToString) -> String {
        format!(
            "{}_{}",
            function_name.to_string(),
            basic_block_id.to_string()
        )
    }

    fn load_vreg(&mut self, dst: impl ToString, vreg: &VirtualRegister) -> CompileResult<()> {
        self.emit(format!("LD {} [0x{}]", dst.to_string(), vreg.0))?;
        Ok(())
    }

    fn store_vreg(&mut self, src: impl ToString, vreg: &VirtualRegister) -> CompileResult<()> {
        self.emit(format!("ST [0x{}] {}", vreg.0, src.to_string()))?;
        Ok(())
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
            let basic_block_label = Self::basic_block_label(&function.name, basic_block.id.0);
            self.sublabel(&basic_block_label)?;

            for instruction in &basic_block.instructions {
                println!("{instruction:?}");
                self.codegen_instruction(instruction)?;
            }

            match basic_block.terminator {
                BasicBlockTerminator::Jmp { next } => {
                    let next_block_label = Self::basic_block_label(&function.name, next.0);
                    self.emit(format!("JMP [.{next_block_label}]"))?;
                }
                BasicBlockTerminator::Branch {
                    condition,
                    then_branch,
                    else_branch,
                } => {
                    let then_block_label = Self::basic_block_label(&function.name, then_branch.0);
                    let else_block_label = Self::basic_block_label(&function.name, else_branch.0);
                    self.load_vreg("R0", &condition)?;
                    self.emit("CMP R0 0")?;
                    self.emit(format!("JCR [.{then_block_label}]"))?;
                    self.emit(format!("JNCR [.{else_block_label}]"))?;
                }
                BasicBlockTerminator::Ret { value } => {
                    if let Some(value) = value {
                        self.load_vreg("R0", &value)?;
                    }
                    self.emit("RET")?;
                }
            }
        }

        Ok(())
    }

    fn codegen_instruction(&mut self, instruction: &IRInstruction) -> CompileResult<()> {
        todo!()
    }
}
