use crate::gen_ir::{Function, IROp, IRType, IR};
use crate::irdump::IRInfo;
use crate::REGS_N;

use std::sync::{LazyLock, Mutex, MutexGuard};

// Quoted from 9cc
// > Register allocator.
//
// > Before this pass, it is assumed that we have infinite number of
// > registers. This pass maps them to a finite number of registers.
// > We actually have only 7 registers.
//
// > We allocate registers only within a single expression. In other
// > words, there are no registers that live beyond semicolons.
// > This design choice simplifies the implementation a lot, since
// > practically we don't have to think about the case in which
// > registers are exhausted and need to be spilled to memory.

static USED: LazyLock<Mutex<[bool; REGS_N]>> = LazyLock::new(|| Mutex::new([false; REGS_N]));
static REG_MAP: LazyLock<Mutex<[Option<usize>; 8192]>> =
    LazyLock::new(|| Mutex::new([None; 8192]));

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(|e| e.into_inner())
}

fn used_get(i: usize) -> bool {
    lock(&USED)[i]
}

fn used_set(i: usize, val: bool) {
    lock(&USED)[i] = val;
}

fn reg_map_get(i: usize) -> Option<usize> {
    lock(&REG_MAP).get(i).copied().flatten()
}

fn reg_map_set(i: usize, val: usize) {
    lock(&REG_MAP)[i] = Some(val);
}

fn alloc(ir_reg: usize) -> usize {
    if lock(&REG_MAP).len() <= ir_reg {
        panic!("program too big");
    }

    if let Some(r) = reg_map_get(ir_reg) {
        assert!(used_get(r));
        return r;
    }

    for i in 0..REGS_N {
        if used_get(i) {
            continue;
        }
        reg_map_set(ir_reg, i);
        used_set(i, true);
        return i;
    }
    panic!("register exhauseted: {}", ir_reg);
}

fn visit(irv: &mut Vec<IR>) {
    use self::IRType::*;

    for item in irv {
        let mut ir = item.clone();
        let info = &IRInfo::from(&ir.op);

        match info.ty {
            Reg | RegImm | RegLabel | LabelAddr => {
                ir.lhs = Some(alloc(ir.lhs.expect("missing lhs")))
            }
            Mem | RegReg => {
                ir.lhs = Some(alloc(ir.lhs.expect("missing lhs")));
                ir.rhs = Some(alloc(ir.rhs.expect("missing rhs")));
            }
            Call => {
                ir.lhs = Some(alloc(ir.lhs.expect("missing lhs")));
                match ir.op {
                    IROp::Call(name, nargs, args) => {
                        let mut args_new: [usize; 6] = [0; 6];
                        for i in 0..nargs {
                            args_new[i] = alloc(args[i]);
                        }
                        ir.op = IROp::Call(name, nargs, args_new);
                    }
                    _ => unreachable!(),
                }
            }
            _ => (),
        }

        if ir.op == IROp::Kill {
            let lhs = ir.lhs.expect("missing lhs");
            assert!(used_get(lhs));
            used_set(lhs, false);
            ir.op = IROp::Nop;
        }
        *item = ir;
    }
}

pub fn alloc_regs(fns: &mut Vec<Function>) {
    for f in fns {
        *lock(&USED) = [false; REGS_N];

        visit(&mut f.ir);
    }
}
