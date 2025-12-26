use crate::{
    error::CompileResult,
    hir::instructions::{HIRGlobal, HIRProgram},
    hir::{
        helpers::lower_type,
        symbols::{Symbol, SymbolKind},
    },
    parser::ast::ASTProgram,
};

use super::SemanticAnalysis;

impl SemanticAnalysis {
    /// Analyze AST program and lower it to HIR
    ///
    /// # Errors
    /// Returns error if there are semantic issues
    pub fn analyze_program(&mut self, program: &ASTProgram) -> CompileResult<HIRProgram> {
        let scope = self.ctx.scope.enter();

        let mut globals = Vec::with_capacity(program.globals.len());
        for global in &program.globals {
            let type_id = self.ctx.types.entry(lower_type(global.ty));
            let symbol = self.ctx.symbols.allocate(Symbol {
                name: global.name.clone(),
                kind: SymbolKind::Global { address: global.at },
                ty: type_id,
            });
            scope.allocate(global.name.clone(), symbol, &global.span)?;
            globals.push(HIRGlobal {
                symbol,
                type_id,
                at: global.at as usize,
            });
        }

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
            globals,
        })
    }
}
