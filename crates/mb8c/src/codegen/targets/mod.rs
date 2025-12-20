use mb8::Mb8Codegen;

use crate::{ir::IRProgram, pipe::CompilerPipe};

pub mod mb8;

#[derive(Debug)]
pub struct Codegen {}

impl CompilerPipe for Codegen {
    type Prev = IRProgram;
    type Next = String;

    fn execute(
        prev: &Self::Prev,
    ) -> crate::error::CompileResult<Self::Next, Vec<crate::error::CompileError>> {
        let mut mb8 = Mb8Codegen::default();
        let asm = mb8.codegen(prev).map_err(|err| vec![err])?;
        Ok(asm)
    }
}
