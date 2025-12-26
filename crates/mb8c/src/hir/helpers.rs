use crate::{
    context::{types::TypeKind, TypeId},
    hir::instructions::HIRExpr,
    parser::ast::ASTType,
};

#[must_use]
pub fn lower_type(ty: ASTType) -> TypeKind {
    match ty {
        ASTType::Void => TypeKind::Void,
        ASTType::Unsigned8 => TypeKind::Unsigned8,
    }
}

#[must_use]
pub fn fetch_expr_type(expr: &HIRExpr) -> TypeId {
    match expr {
        HIRExpr::Var { ty, .. }
        | HIRExpr::Literal { ty, .. }
        | HIRExpr::Binary { ty, .. }
        | HIRExpr::Unary { ty, .. }
        | HIRExpr::Call { ty, .. } => *ty,
    }
}
