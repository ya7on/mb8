use crate::{ast::ASTProgram, error::CompileResult, hir::HIRProgram};

use super::SemanticAnalysis;

impl SemanticAnalysis {
    /// Analyze AST program and lower it to HIR
    ///
    /// # Errors
    /// Returns error if there are semantic issues
    pub fn analyze_program(&mut self, program: &ASTProgram) -> CompileResult<HIRProgram> {
        self.ctx.scope.enter();

        let mut functions = Vec::with_capacity(program.functions.len());

        // 1st iteration. collect function names
        for function in &program.functions {
            self.collect_function(function)?;
        }

        // 2nd iteration
        for function in &program.functions {
            let hir_function = self.analyze_function(function)?;
            functions.push(hir_function);
        }

        Ok(HIRProgram {
            symbols: self.ctx.symbols.clone(),
            types: self.ctx.types.clone(),
            functions,
        })
    }
}
