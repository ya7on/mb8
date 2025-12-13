use std::collections::HashMap;

use crate::{error::CompileResult, hir::SymbolId};

#[derive(Debug, Default)]
pub struct ScopeStack {
    stack: Vec<Scope>,
}

impl ScopeStack {
    pub fn enter(&mut self) {
        self.stack.push(Scope::default());
    }

    pub fn exit(&mut self) {
        self.stack.pop();
    }

    pub fn current(&mut self) -> &mut Scope {
        self.stack.last_mut().unwrap()
    }
}

#[derive(Debug, Default)]
pub struct Scope {
    symbols: HashMap<String, SymbolId>,
}

impl Scope {
    pub fn allocate(&mut self, name: String, id: SymbolId) -> CompileResult<()> {
        if self.symbols.contains_key(&name) {
            Err(todo!())
        } else {
            self.symbols.insert(name, id);
            Ok(())
        }
    }
}
