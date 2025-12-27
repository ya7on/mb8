use crate::{
    context::symbols::{Symbol, SymbolKind},
    error::CompileResult,
    hir::{helpers::lower_type, instructions::HIRProgram},
    parser::ast::ASTProgram,
};

use super::HIRLowerer;

impl HIRLowerer {
    /// Analyze AST program and lower it to HIR
    ///
    /// # Errors
    /// Returns error if there are semantic issues
    pub fn analyze_program(&mut self, program: &ASTProgram) -> CompileResult<HIRProgram> {
        let scope = self.scope.enter();

        let mut globals = Vec::with_capacity(program.globals.len());
        for global in &program.globals {
            let type_id = self.ctx.type_table.entry(lower_type(global.ty));
            let symbol = self.ctx.symbol_table.allocate(Symbol {
                name: global.name.clone(),
                kind: SymbolKind::Global { address: global.at },
                ty: type_id,
            });
            scope.allocate(global.name.clone(), symbol, &global.span)?;
            globals.push(symbol);
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

        Ok(HIRProgram { functions, globals })
    }
}
