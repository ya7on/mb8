use crate::{
    context::{types::TypeKind, CompileContext, TypeId},
    hir::instructions::HIRExpr,
    parser::ast::ASTType,
};

#[must_use]
pub fn lower_type(ctx: &mut CompileContext, ty: &ASTType) -> TypeId {
    match ty {
        ASTType::Void => ctx.type_table.entry(TypeKind::Void),
        ASTType::Unsigned8 => ctx.type_table.entry(TypeKind::Unsigned8),
        ASTType::Unsigned16 => ctx.type_table.entry(TypeKind::Unsigned16),
        ASTType::Pointer(inner) => {
            let pointee = lower_type(ctx, inner);
            ctx.type_table.entry(TypeKind::Pointer { pointee })
        }
    }
}

#[must_use]
pub fn fetch_expr_type(expr: &HIRExpr) -> TypeId {
    match expr {
        HIRExpr::Var { ty, .. }
        | HIRExpr::Literal { ty, .. }
        | HIRExpr::Binary {
            result_type: ty, ..
        }
        | HIRExpr::Unary { ty, .. }
        | HIRExpr::Call { ty, .. } => *ty,
    }
}
