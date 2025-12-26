use super::{scope::ScopeStack, symbols::SymbolTable, types::TypeTable};

#[derive(Debug, Default)]
pub struct SemanticContext {
    pub symbols: SymbolTable,
    pub types: TypeTable,
    pub scope: ScopeStack,
}
