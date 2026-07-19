use std::collections::HashMap;

use crate::{
    diagnostics::Span,
    error::{AsmError, AsmErrorKind},
    ir::{IRItem, IRProgram},
};

pub fn tag(ir: &IRProgram) -> Result<HashMap<String, u16>, AsmError> {
    let mut labels = HashMap::<String, (u16, Span)>::new();
    let mut counter = ir.origin.as_ref().map_or(0, |origin| origin.value);
    for item in &ir.items {
        match item {
            IRItem::Label(label) => {
                if let Some((_, first)) =
                    labels.insert(label.value.clone(), (counter, label.span.clone()))
                {
                    return Err(AsmError {
                        span: Some(label.span.clone()),
                        kind: AsmErrorKind::DuplicateLabel {
                            label: label.value.clone(),
                            first,
                        },
                    });
                }
            }
            IRItem::Instruction(instruction) => {
                counter = counter.checked_add(2).ok_or_else(|| AsmError {
                    span: Some(instruction.span.clone()),
                    kind: AsmErrorKind::AddressOverflow { current: counter },
                })?;
            }
            IRItem::Data(data) => {
                counter = counter
                    .checked_add(u16::try_from(data.value.len()).map_err(|_| AsmError {
                        span: Some(data.span.clone()),
                        kind: AsmErrorKind::AddressOverflow { current: counter },
                    })?)
                    .ok_or_else(|| AsmError {
                        span: Some(data.span.clone()),
                        kind: AsmErrorKind::AddressOverflow { current: counter },
                    })?;
            }
        }
    }
    Ok(labels
        .into_iter()
        .map(|(label, (address, _))| (label, address))
        .collect())
}
