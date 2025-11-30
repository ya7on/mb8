use mb6::Mb8Asm;

use crate::{
    error::CompileResult,
    ir::{IRFunction, IROpcode},
};

pub mod mb6;

#[derive(Debug)]
pub struct CodeGenerator {
    ir: Vec<IRFunction>,
}

impl CodeGenerator {
    #[must_use]
    pub fn new(ir: Vec<IRFunction>) -> Self {
        CodeGenerator { ir }
    }

    /// # Errors
    /// Returns a `CompileError` if there was an error writing to the code buffer.
    pub fn generate(&self) -> CompileResult<String> {
        let mut code = String::new();
        for func in &self.ir {
            code.push_str(&self.generate_function(func)?);
        }
        Ok(code)
    }

    /// # Errors
    /// Returns a `CompileError` if there was an error writing to the code buffer.
    pub fn generate_function(&self, program: &IRFunction) -> CompileResult<String> {
        let mut result = Mb8Asm::default();
        let func_name = &program.name;

        result.label(func_name)?;
        for instruction in &program.code {
            match &instruction.opcode {
                IROpcode::LoadImm { imm } => {
                    result.ldi("R0", *imm)?;
                }
                IROpcode::Return => {
                    result.ret()?;
                }
                IROpcode::Call { name, args: _ } => {
                    result.call(name)?;
                }
                _ => {}
            }
        }

        Ok(result.code)
    }
}
