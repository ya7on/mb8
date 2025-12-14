use std::collections::HashMap;

use crate::hir::TypeId;

#[derive(Debug, Default)]
pub struct TypeTable {
    types: Vec<TypeKind>,
    index: HashMap<TypeKind, TypeId>,
}

impl TypeTable {
    pub fn entry(&mut self, kind: TypeKind) -> TypeId {
        if let Some(id) = self.index.get(&kind) {
            *id
        } else {
            let id = TypeId(self.types.len());
            self.types.push(kind.clone());
            self.index.insert(kind, id);
            id
        }
    }

    #[must_use] pub fn lookup(&self, type_id: TypeId) -> Option<&TypeKind> {
        self.types.get(type_id.0)
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum TypeKind {
    Void,
    Int,
    Char,
    Function { params: Vec<TypeId>, ret: TypeId },
}
