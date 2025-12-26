use crate::{
    error::CompileResult,
    hir::instructions::HIRFunction,
    ir::{bb::BasicBlockTable, instructions::IRFunction},
};

use super::IRLowerer;

impl IRLowerer {
    pub fn lower_function(&mut self, function: &HIRFunction) -> CompileResult<IRFunction> {
        let mut bbtable = BasicBlockTable::default();
        let mut basic_blocks = Vec::new();

        let mut builder = bbtable.bb();
        for stmt in &function.body {
            let (maybe_builder, blocks) = self.lower_stmt(stmt, builder, &mut bbtable)?;
            basic_blocks.extend(blocks);
            let Some(new_builder) = maybe_builder else {
                break;
            };
            builder = new_builder;
        }
        Ok(IRFunction {
            id: function.id,
            basic_blocks,
        })
    }
}
