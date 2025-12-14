use crate::hir::{SymbolId, TypeId};

#[derive(Debug, Default)]
pub struct SymbolTable {
    pub symbols: Vec<Symbol>,
}

impl SymbolTable {
    pub fn allocate(&mut self, symbol: Symbol) -> SymbolId {
        let id = self.symbols.len();
        self.symbols.push(symbol);
        SymbolId(id)
    }

    pub fn lookup(&self, symbol_id: SymbolId) -> Option<Symbol> {
        self.symbols.get(symbol_id.0).cloned()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    Variable,
    Function,
    Parameter,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub ty: TypeId,
}
