#![allow(clippy::missing_errors_doc, clippy::needless_pass_by_value)]

use std::fmt::Debug;

use crate::error::{CompileError, CompileResult};

#[derive(Debug)]
pub struct CompilePipeline<T>
where
    T: CompilerPipe + Debug,
{
    pub result: T::Next,
}

impl<T> CompilePipeline<T>
where
    T: CompilerPipe + Debug,
{
    pub fn init(input: T::Prev) -> CompileResult<CompilePipeline<T>, Vec<CompileError>> {
        Ok(Self {
            result: T::execute(&input)?,
        })
    }

    pub fn and_next<N>(&mut self) -> CompileResult<CompilePipeline<N>, Vec<CompileError>>
    where
        N: CompilerPipe<Prev = T::Next> + Debug,
    {
        Ok(CompilePipeline {
            result: N::execute(&self.result)?,
        })
    }

    pub fn finish(self) -> CompileResult<T::Next, Vec<CompileError>> {
        Ok(self.result)
    }
}

pub trait CompilerPipe {
    type Prev;
    type Next;

    fn execute(prev: &Self::Prev) -> CompileResult<Self::Next, Vec<CompileError>>;
}
