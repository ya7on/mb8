use mb8_isa::registers::Register;

use crate::{
    ast::{ASTItem, ASTProgram, DataSource, Operand},
    diagnostics::{Diagnostic, DiagnosticKind, Severity},
    pass::{AssemblerPass, PassContext},
};

pub(crate) struct RegisterAliasPass;

impl AssemblerPass for RegisterAliasPass {
    type Input = ASTProgram;
    type Output = ASTProgram;

    fn run(&mut self, input: Self::Input, context: &mut PassContext<'_>) -> Option<Self::Output> {
        for item in &input.items {
            let ASTItem::Instruction(instruction) = item else {
                continue;
            };

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
                let alias = match register {
                    Register::R0 => Register::A,
                    Register::R9 => Register::IH,
                    Register::R10 => Register::IL,
                    Register::R11 => Register::FPH,
                    Register::R12 => Register::FPL,
                    Register::R13 => Register::SPH,
                    Register::R14 => Register::SPL,
                    Register::R15 => Register::F,
                    Register::R1
                    | Register::R2
                    | Register::R3
                    | Register::R4
                    | Register::R5
                    | Register::R6
                    | Register::R7
                    | Register::R8
                    | Register::A
                    | Register::IH
                    | Register::IL
                    | Register::FPH
                    | Register::FPL
                    | Register::SPH
                    | Register::SPL
                    | Register::F => continue,
                };

                context.emit(Diagnostic {
                    severity: Severity::Warning,
                    span: Some(instruction.span.clone()),
                    kind: DiagnosticKind::RegisterAlias { register, alias },
                });
            }
        }

        Some(input)
    }
}
