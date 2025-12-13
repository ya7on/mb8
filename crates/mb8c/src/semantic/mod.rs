use context::Context;
use symbols::{Symbol, SymbolKind};
use types::TypeKind;

use crate::{
    ast::{ASTFunction, ASTProgram, ASTType},
    error::CompileResult,
    hir::HIRProgram,
};

pub mod context;
pub mod scope;
pub mod symbols;
pub mod types;

pub fn lower_type(ty: ASTType) -> TypeKind {
    match ty {
        ASTType::Void => TypeKind::Void,
        ASTType::Char => TypeKind::Char,
        ASTType::Int => TypeKind::Int,
    }
}

pub fn collect_function(ctx: &mut Context, ast: &ASTFunction) -> CompileResult<()> {
    let params = ast
        .params
        .iter()
        .map(|(_name, ty)| ctx.types.entry(lower_type(*ty)))
        .collect();
    let ret = ctx.types.entry(lower_type(ast.return_type));
    let type_id = ctx.types.entry(TypeKind::Function { params, ret });

    let symbol = ctx.symbols.allocate(Symbol {
        name: ast.name.clone(),
        kind: SymbolKind::Function,
        ty: type_id,
    });

    ctx.scope.current();

    todo!()
}

pub fn analyze(ast: &ASTProgram) -> CompileResult<HIRProgram> {
    let mut context = Context::default();

    // Global scope
    context.scope.enter();

    let mut hir = HIRProgram {
        functions: Vec::with_capacity(ast.functions.len()),
    };

    // 1st iteration. collect function names
    for function in &ast.functions {
        collect_function(&mut context, function)?;
    }

    Ok(hir)
}
