use crate::{
    ast::ASTFunction,
    error::CompileResult,
    hir::{HIRFunction, SymbolId},
    semantic::{
        context::Context,
        helpers::lower_type,
        symbols::{Symbol, SymbolKind},
        types::TypeKind,
    },
};

use super::stmt::analyze_stmt;

pub fn collect_function(ctx: &mut Context, function: &ASTFunction) -> CompileResult<()> {
    let params = function
        .params
        .iter()
        .map(|(_name, ty)| ctx.types.entry(lower_type(*ty)))
        .collect();
    let ret = ctx.types.entry(lower_type(function.return_type));
    let type_id = ctx.types.entry(TypeKind::Function { params, ret });

    let symbol = ctx.symbols.allocate(Symbol {
        name: function.name.clone(),
        kind: SymbolKind::Function,
        ty: type_id,
    });

    let scope = ctx.scope.current();
    scope.allocate(function.name.clone(), symbol, function.span.clone())?;

    Ok(())
}

pub fn analyze_function(ctx: &mut Context, function: &ASTFunction) -> CompileResult<HIRFunction> {
    let scope = ctx.scope.enter();

    let mut params = Vec::with_capacity(function.params.len());
    // Collect params
    for (name, ty) in &function.params {
        let type_id = ctx.types.entry(lower_type(*ty));
        let symbol = ctx.symbols.allocate(Symbol {
            name: name.to_owned(),
            kind: SymbolKind::Parameter,
            ty: type_id,
        });
        scope.allocate(name.to_owned(), symbol, function.span.clone())?;
        params.push(symbol);
    }

    let return_type_id = ctx.types.entry(lower_type(function.return_type));

    let body = analyze_stmt(ctx, &function.body, return_type_id)?;

    Ok(HIRFunction {
        id: SymbolId(1),
        params,
        body: vec![body],
    })
}
