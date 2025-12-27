use crate::{
    context::{
        symbols::{Symbol, SymbolKind},
        types::TypeKind,
    },
    error::{CompileError, CompileResult},
    hir::{helpers::lower_type, instructions::HIRFunction},
    parser::ast::ASTFunction,
};

use super::HIRLowerer;

impl HIRLowerer {
    /// First iteration through AST functions to collect their names
    ///
    /// # Errors
    /// Returns error if there are semantic issues
    pub fn collect_function(&mut self, function: &ASTFunction) -> CompileResult<()> {
        let params = function
            .params
            .iter()
            .map(|(_name, ty)| self.ctx.type_table.entry(lower_type(*ty)))
            .collect();
        let ret = self.ctx.type_table.entry(lower_type(function.return_type));
        let type_id = self
            .ctx
            .type_table
            .entry(TypeKind::Function { params, ret });

        let symbol = self.ctx.symbol_table.allocate(Symbol {
            name: function.name.clone(),
            kind: SymbolKind::Function,
            ty: type_id,
        });

        let scope = self.scope.current();
        scope.allocate(function.name.clone(), symbol, &function.span)?;

        Ok(())
    }

    /// Deep analysis of AST function and loweing it to HIR
    ///
    /// # Errors
    /// Returns error if there are semantic issues
    pub fn analyze_function(&mut self, function: &ASTFunction) -> CompileResult<HIRFunction> {
        let id = self
            .scope
            .lookup(&function.name)
            .ok_or_else(|| CompileError::InternalError {
                message: "Cannot find function".to_string(),
            })?;

        let scope = self.scope.enter();

        let mut params = Vec::with_capacity(function.params.len());
        // Collect params
        for (name, ty) in &function.params {
            let symbol = self.ctx.allocate_parameter(name, lower_type(*ty));
            scope.allocate(name.to_owned(), symbol, &function.span)?;
            params.push(symbol);
        }

        let mut locals = Vec::with_capacity(function.vars.len());
        // Collects local variables
        for (name, ty) in &function.vars {
            let symbol = self.ctx.allocate_local(name, lower_type(*ty));
            scope.allocate(name.to_owned(), symbol, &function.span)?;
            locals.push(symbol);
        }

        let return_type_id = self.ctx.type_table.entry(lower_type(function.return_type));

        let body = self.analyze_stmt(&function.body, return_type_id)?;

        let hir = HIRFunction {
            id,
            params,
            locals,
            body: vec![body],
        };

        // TODO: Control flow checks

        Ok(hir)
    }
}
