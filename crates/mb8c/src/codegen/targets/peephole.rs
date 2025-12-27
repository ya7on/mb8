use crate::{
    error::{CompileError, CompileResult},
    pipeline::CompilerPipe,
};

use super::asm::Mb8Asm;

#[derive(Debug, Default)]
pub struct Mb8Peephole {
    result: Vec<Mb8Asm>,
}

impl Mb8Peephole {
    /// Remove a matching trailing `PUSH`/`POP` pair.
    ///
    /// # Errors
    /// Currently infallible but keeps the result type for future validation steps.
    pub fn remove_push_pop(&mut self) -> CompileResult<bool> {
        if self.result.len() < 2 {
            return Ok(false);
        }

        let last = self.result.last();
        let prev = self.result.get(self.result.len() - 2);

        match (prev, last) {
            (
                Some(Mb8Asm::Push {
                    register: push_register,
                }),
                Some(Mb8Asm::Pop {
                    register: pop_register,
                }),
            ) if push_register == pop_register => {
                self.result.pop();
                self.result.pop();
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    /// Remove a matching trailing `PUSH`/`POP` pair.
    ///
    /// # Errors
    /// Currently infallible but keeps the result type for future validation steps.
    pub fn use_mov_instead_of_push_pop(&mut self) -> CompileResult<bool> {
        if self.result.len() < 2 {
            return Ok(false);
        }

        let last = self.result.last().cloned();
        let prev = self.result.get(self.result.len() - 2).cloned();

        match (prev, last) {
            (
                Some(Mb8Asm::Push {
                    register: push_register,
                }),
                Some(Mb8Asm::Pop {
                    register: pop_register,
                }),
            ) if push_register != pop_register => {
                self.result.pop();
                self.result.pop();
                self.result.push(Mb8Asm::Mov {
                    dst: pop_register,
                    src: push_register,
                });
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    /// Apply peephole optimizations repeatedly until no further change occurs.
    ///
    /// # Errors
    /// Currently infallible but keeps the result type for future validation steps.
    pub fn check_output(&mut self) -> CompileResult<()> {
        loop {
            let updated = self.remove_push_pop()? || self.use_mov_instead_of_push_pop()?;
            if !updated {
                break;
            }
        }

        Ok(())
    }

    /// Run peephole passes over the provided instructions as they are appended.
    ///
    /// # Errors
    /// Currently infallible but keeps the result type for future validation steps.
    pub fn pass(&mut self, input: &[Mb8Asm]) -> CompileResult<()> {
        for instruction in input {
            self.result.push(instruction.clone());
            self.check_output()?;
        }
        Ok(())
    }
}

impl CompilerPipe for Mb8Peephole {
    type Prev = Vec<Mb8Asm>;
    type Next = Vec<Mb8Asm>;

    fn execute(prev: &Self::Prev) -> CompileResult<Self::Next, Vec<CompileError>> {
        let mut peephole = Mb8Peephole::default();
        peephole.pass(prev).map_err(|err| vec![err])?;
        Ok(peephole.result)
    }
}
