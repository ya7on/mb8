use crate::{
    error::{CompileError, CompileResult},
    hir::instructions::HIRFunction,
    ir::{bb::BasicBlockTable, instructions::IRFunction},
};

use super::IRLowerer;

impl IRLowerer {
    /// Lower a high-level function into IR basic blocks.
    ///
    /// # Errors
    /// Returns an error when lowering of contained statements fails.
    pub fn lower_function(&mut self, function: &HIRFunction) -> CompileResult<IRFunction> {
        let mut bbtable = BasicBlockTable::default();
        let mut basic_blocks = Vec::new();

        let mut builder = Some(bbtable.bb());
        for stmt in &function.body {
            let Some(active_builder) = builder else {
                break;
            };

            let (active_builder, blocks) = self.lower_stmt(stmt, active_builder, &mut bbtable)?;
            basic_blocks.extend(blocks);
            builder = active_builder;
        }
        if builder.is_some() {
            let symbol = self.ctx.lookup(function.id).ok_or_else(|| todo!())?;
            return Err(CompileError::InternalError {
                message: format!(
                    "Function {:?} does not end with a return statement",
                    symbol.name
                ),
            });
        }

        Ok(IRFunction {
            id: function.id,
            basic_blocks,
            params: function.params.clone(),
            locals: function.locals.clone(),
        })
    }
}
