use super::context::SemanticContext;

pub mod expr;
pub mod function;
pub mod program;
pub mod stmt;

#[derive(Debug, Default)]
pub struct SemanticAnalysis {
    pub ctx: SemanticContext,
}
