use std::collections::HashMap;

use crate::{
    ast::Span,
    error::{CompileError, CompileResult},
    hir::SymbolId,
};

#[derive(Debug, Default)]
pub struct ScopeStack {
    stack: Vec<Scope>,
}

impl ScopeStack {
    pub fn enter(&mut self) -> &mut Scope {
        self.stack.push(Scope::default());
        self.current()
    }

    pub fn exit(&mut self) {
        self.stack.pop();
    }

    pub fn current(&mut self) -> &mut Scope {
        self.stack.last_mut().unwrap()
    }

    pub fn lookup(&self, name: &str) -> Option<SymbolId> {
        self.stack
            .iter()
            .rev()
            .find_map(|scope| scope.symbols.get(name).copied())
    }
}

#[derive(Debug, Default)]
pub struct Scope {
    symbols: HashMap<String, SymbolId>,
}

impl Scope {
    pub fn allocate(&mut self, name: String, id: SymbolId, span: Span) -> CompileResult<()> {
        if self.symbols.contains_key(&name) {
            Err(CompileError::DuplicateSymbol {
                start: span.start,
                end: span.end,
                symbol: name,
            })
        } else {
            self.symbols.insert(name, id);
            Ok(())
        }
    }
}
