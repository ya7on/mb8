use crate::{error::CompileResult, hir::instructions::HIRProgram, ir::instructions::IRProgram};

use super::IRLowerer;

impl IRLowerer {
    /// Lower a full HIR program into IR.
    ///
    /// # Errors
    /// Returns an error when any function lowering fails.
    pub fn lower_program(&mut self, program: &HIRProgram) -> CompileResult<IRProgram> {
        let mut functions = Vec::with_capacity(program.functions.len());

        for function in &program.functions {
            functions.push(self.lower_function(function)?);
        }

        Ok(IRProgram {
            functions,
            globals: program.globals.clone(),
        })
    }
}
