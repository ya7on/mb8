use crate::{
    ast::{ASTItem, ASTProgram, DataSource, Operand},
    diagnostics::{Diagnostic, DiagnosticKind, Severity},
    instructions::HANDLERS,
    pass::{AssemblerPass, PassContext},
};

pub(crate) struct ScratchRegisterPass;

impl AssemblerPass for ScratchRegisterPass {
    type Input = ASTProgram;
    type Output = ASTProgram;

    fn run(&mut self, input: Self::Input, context: &mut PassContext<'_>) -> Option<Self::Output> {
        for (id, item) in input.items.iter().enumerate() {
            let ASTItem::Instruction(instruction) = item else {
                continue;
            };

            let Some(definition) = HANDLERS
                .iter()
                .filter(|definition| definition.mnemonic == instruction.value.mnemonic)
                .find(|definition| {
                    (definition.handler)(&instruction.value, &instruction.span, id).is_some()
                })
            else {
                continue;
            };

            let scratch = definition.effect.scratch;
            let registers = instruction.value.operands.iter().flat_map(|operand| {
                let registers = match operand {
                    Operand::Raw(source) | Operand::MemoryWrapped(source) => match source {
                        DataSource::Register(register) => [Some(*register), None],
                        DataSource::RegisterPair(hi, lo) => [Some(*hi), Some(*lo)],
                        DataSource::Immediate(_)
                        | DataSource::Constant(_)
                        | DataSource::Label(_) => [None, None],
                    },
                    Operand::MemoryOffset { hi, lo, .. } => [Some(*hi), Some(*lo)],
                };
                registers.into_iter().flatten()
            });

            for register in registers {
                if scratch.contains(register) {
                    context.emit(Diagnostic {
                        severity: Severity::Error,
                        span: Some(instruction.span.clone()),
                        kind: DiagnosticKind::ScratchRegisterConflict {
                            mnemonic: instruction.value.mnemonic.clone(),
                            register,
                        },
                    });
                }
            }
        }

        Some(input)
    }
}
