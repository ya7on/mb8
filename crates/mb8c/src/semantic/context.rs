use super::{scope::ScopeStack, symbols::SymbolTable, types::TypeTable};

#[derive(Debug, Default)]
pub struct Context {
    pub symbols: SymbolTable,
    pub types: TypeTable,
    pub scope: ScopeStack,
}
