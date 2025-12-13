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
}

#[derive(Debug)]
pub enum SymbolKind {
    Variable,
    Function,
    Parameter,
}

#[derive(Debug)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub ty: TypeId,
}
