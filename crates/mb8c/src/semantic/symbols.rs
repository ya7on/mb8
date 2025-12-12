use std::collections::HashMap;

use crate::{
    ast::Type,
    error::{CompileError, CompileResult},
};

#[derive(Debug)]
pub struct FunctionSignature {
    pub return_type: Type,
    pub param_types: Vec<Type>,
}

pub type Functions = HashMap<String, FunctionSignature>;

#[derive(Debug)]
pub struct Symbols {
    scopes: Vec<HashMap<String, Type>>,
}

impl Default for Symbols {
    fn default() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }
}

impl Symbols {
    /// Inserts a symbol into the current scope.
    ///
    /// # Errors
    /// Returns an error if the symbol already exists in the current scope.
    pub fn insert(&mut self, name: String, ty: Type) -> CompileResult<()> {
        let scope = self
            .scopes
            .last_mut()
            .ok_or_else(|| CompileError::InternalError {
                message: "Cannot find function scope".to_owned(),
            })?;
        if scope.contains_key(&name) {
            return Err(CompileError::DuplicateVariable { name });
        }
        scope.insert(name, ty);
        Ok(())
    }

    #[must_use]
    pub fn lookup_var(&self, name: &str) -> Option<Type> {
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(*ty);
            }
        }
        None
    }

    /// Enters a new scope.
    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// Leaves the current scope.
    pub fn leave_scope(&mut self) {
        self.scopes.pop();
    }
}
