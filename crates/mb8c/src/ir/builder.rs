use std::collections::HashMap;

use crate::parser::ast::Type;

use super::{IRFunction, IRInstruction, IROpcode, LocalInfo, Reg};

#[derive(Debug)]
pub struct IRBuilder {
    pub func: IRFunction,
    pub next_reg: u32,
    pub next_label: u32,
    pub locals_map: HashMap<String, u32>,
}

impl IRBuilder {
    #[must_use]
    pub fn new(name: String, return_type: Type) -> Self {
        Self {
            func: IRFunction {
                name,
                return_type,
                locals: Vec::new(),
                code: Vec::new(),
            },
            next_reg: 0,
            next_label: 0,
            locals_map: HashMap::new(),
        }
    }

    pub fn new_reg(&mut self) -> Reg {
        let r = Reg(self.next_reg);
        self.next_reg += 1;
        r
    }

    pub fn new_label(&mut self) -> u32 {
        let label = self.next_label;
        self.next_label += 1;
        label
    }

    pub fn emit(
        &mut self,
        opcode: IROpcode,
        dst: Option<Reg>,
        src1: Option<Reg>,
        src2: Option<Reg>,
    ) {
        self.func.code.push(IRInstruction {
            opcode,
            dst,
            src1,
            src2,
        });
    }

    pub fn add_local(&mut self, name: String, ty: Type) -> u32 {
        let id = self.func.locals.len() as u32;
        self.func.locals.push(LocalInfo {
            name: name.clone(),
            ty,
        });
        self.locals_map.insert(name, id);
        id
    }
}
