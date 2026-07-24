use crate::{
    ast::{ASTInstruction, ASTItem, ASTProgram, Directive},
    diagnostics::Spanned,
    error::{AsmError, AsmErrorKind},
    instructions::HANDLERS,
    ir::{IRItem, IRProgram},
};

pub fn desugar_instruction(
    instruction: &Spanned<ASTInstruction>,
    id: usize,
) -> Result<Vec<IRItem>, AsmError> {
    HANDLERS
        .iter()
        .filter(|definition| definition.mnemonic == instruction.value.mnemonic)
        .find_map(|definition| (definition.handler)(&instruction.value, &instruction.span, id))
        .ok_or_else(|| AsmError {
            span: Some(instruction.span.clone()),
            kind: AsmErrorKind::UnsupportedInstruction {
                mnemonic: instruction.value.mnemonic.clone(),
                operands: instruction.value.operands.clone(),
            },
        })
}

pub fn desugar(ast: &ASTProgram) -> Result<IRProgram, AsmError> {
    let mut result = IRProgram {
        origin: None,
        items: Vec::new(),
    };
    for (id, item) in ast.items.iter().enumerate() {
        match item {
            ASTItem::Instruction(inst) => {
                result.items.extend(desugar_instruction(inst, id)?);
            }
            ASTItem::Label(label) => {
                result.items.push(IRItem::Label(label.clone()));
            }
            ASTItem::Directive(directive) => match &directive.value {
                Directive::Origin(address) => {
                    if let Some(origin) = &result.origin {
                        return Err(AsmError {
                            span: Some(directive.span.clone()),
                            kind: AsmErrorKind::DuplicateOrigin {
                                first: origin.span.clone(),
                            },
                        });
                    }
                    result.origin = Some(Spanned {
                        value: *address,
                        span: directive.span.clone(),
                    });
                }
                Directive::Data(bytes) => result.items.push(IRItem::Data(Spanned {
                    value: bytes.clone(),
                    span: directive.span.clone(),
                })),
                Directive::Ascii(text) => result.items.push(IRItem::Data(Spanned {
                    value: text.as_bytes().to_vec(),
                    span: directive.span.clone(),
                })),
                Directive::Include(_) => {
                    return Err(AsmError {
                        span: Some(directive.span.clone()),
                        kind: AsmErrorKind::UnexpectedDirective {
                            directive: directive.value.clone(),
                        },
                    });
                }
            },
        }
    }
    Ok(result)
}
