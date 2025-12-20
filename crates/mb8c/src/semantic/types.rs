use std::collections::HashMap;

use crate::hir::TypeId;

#[derive(Debug, Default, Clone)]
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

    #[must_use]
    pub fn lookup(&self, type_id: TypeId) -> Option<&TypeKind> {
        self.types.get(type_id.0)
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Default)]
pub enum TypeKind {
    #[default]
    Void,
    Bool,
    Unsigned8,
    Function {
        params: Vec<TypeId>,
        ret: TypeId,
    },
}

impl TypeKind {
    #[must_use]
    pub fn size(&self) -> usize {
        match self {
            Self::Void | Self::Function { .. } => 0,
            Self::Bool | Self::Unsigned8 => 1,
        }
    }
}
