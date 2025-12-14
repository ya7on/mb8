use crate::{
    ast::ASTType,
    hir::{HIRExpr, TypeId},
};

use super::types::TypeKind;

pub fn lower_type(ty: ASTType) -> TypeKind {
    match ty {
        ASTType::Void => TypeKind::Void,
        ASTType::Char => TypeKind::Char,
        ASTType::Int => TypeKind::Int,
    }
}

pub fn fetch_expr_type(expr: &HIRExpr) -> TypeId {
    match expr {
        HIRExpr::Var { ty, .. } => ty.clone(),
        HIRExpr::Literal { ty, .. } => ty.clone(),
        HIRExpr::Binary { ty, .. } => ty.clone(),
        HIRExpr::Unary { ty, .. } => ty.clone(),
        HIRExpr::Call { ty, .. } => ty.clone(),
        HIRExpr::Assign { ty, .. } => ty.clone(),
    }
}
