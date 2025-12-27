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

    pub fn check_output(&mut self) -> CompileResult<()> {
        loop {
            let updated = self.remove_push_pop()?;
            if !updated {
                break;
            }
        }

        Ok(())
    }

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
