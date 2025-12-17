use crate::{
    ast::ASTFunction,
    error::CompileResult,
    hir::{HIRFunction, HIRFunctionParam, SymbolId},
    semantic::{
        helpers::lower_type,
        symbols::{Symbol, SymbolKind},
        types::TypeKind,
        SemanticAnalysis,
    },
};

impl SemanticAnalysis {
    /// First iteration through AST functions to collect their names
    ///
    /// # Errors
    /// Returns error if there are semantic issues
    pub fn collect_function(&mut self, function: &ASTFunction) -> CompileResult<()> {
        let params = function
            .params
            .iter()
            .map(|(_name, ty)| self.ctx.types.entry(lower_type(*ty)))
            .collect();
        let ret = self.ctx.types.entry(lower_type(function.return_type));
        let type_id = self.ctx.types.entry(TypeKind::Function { params, ret });

        let symbol = self.ctx.symbols.allocate(Symbol {
            name: function.name.clone(),
            kind: SymbolKind::Function,
            ty: type_id,
        });

        let scope = self.ctx.scope.current();
        scope.allocate(function.name.clone(), symbol, &function.span)?;

        Ok(())
    }

    /// Deep analysis of AST function and loweing it to HIR
    ///
    /// # Errors
    /// Returns error if there are semantic issues
    pub fn analyze_function(&mut self, function: &ASTFunction) -> CompileResult<HIRFunction> {
        let scope = self.ctx.scope.enter();
        let mut size = 0;

        let mut params = Vec::with_capacity(function.params.len());
        // Collect params
        for (name, ty) in &function.params {
            let hir_type = lower_type(*ty);
            let type_id = self.ctx.types.entry(hir_type.clone());
            let symbol = self.ctx.symbols.allocate(Symbol {
                name: name.to_owned(),
                kind: SymbolKind::Parameter,
                ty: type_id,
            });
            scope.allocate(name.to_owned(), symbol, &function.span)?;
            params.push(HIRFunctionParam {
                symbol,
                size: hir_type.size() as usize,
                offset: size,
            });
            size += hir_type.size() as usize;
        }

        // Collects local vaers
        for (name, ty) in &function.vars {
            let hir_type = lower_type(*ty);
            let type_id = self.ctx.types.entry(hir_type.clone());
            let symbol = self.ctx.symbols.allocate(Symbol {
                name: name.to_owned(),
                kind: SymbolKind::Variable,
                ty: type_id,
            });
            scope.allocate(name.to_owned(), symbol, &function.span)?;
            params.push(HIRFunctionParam {
                symbol,
                size: hir_type.size() as usize,
                offset: size,
            });
            size += hir_type.size() as usize;
        }

        let return_type_id = self.ctx.types.entry(lower_type(function.return_type));

        let body = self.analyze_stmt(&function.body, return_type_id)?;

        let hir = HIRFunction {
            id: SymbolId(1),
            params,
            body: vec![body],
            params_size: size,
        };

        // TODO: Control flow checks

        Ok(hir)
    }
}
