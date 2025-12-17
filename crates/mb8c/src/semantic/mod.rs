use context::SemanticContext;

pub mod analysis;
pub mod context;
pub mod helpers;
pub mod scope;
pub mod symbols;
pub mod types;

#[derive(Debug, Default)]
pub struct SemanticAnalysis {
    pub ctx: SemanticContext,
}
