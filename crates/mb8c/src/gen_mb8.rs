use crate::gen_ir::{Function, IROp};
use crate::{Scope, Var, REGS_N};

use std::sync::{LazyLock, Mutex, MutexGuard};

const REGS: [&str; REGS_N] = ["R0", "R1", "R2", "R3", "R4", "R7", "R0"];

static LABEL: LazyLock<Mutex<usize>> = LazyLock::new(|| Mutex::new(0));

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

macro_rules! emit {
    ($fmt:expr) => (print!(concat!("\t", $fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!("\t", $fmt, "\n"), $($arg)*));
}

fn emit_cmp_set_bool(lhs: usize, rhs: usize, cond: &'static str) {
    let id = {
        let mut g = lock(&LABEL);
        let cur = *g;
        *g += 1;
        cur
    };

    let reg_l = REGS[lhs];
    let reg_r = REGS[rhs];

    emit!("CMP {} {}", reg_l, reg_r);

    emit!("ZERO {}", reg_l);

    match cond {
        "eq" => {
            emit!("JNZR .Lend_cmp_{}", id);
        }
        "ne" => {
            emit!("JZR .Lend_cmp_{}", id);
        }
        _ => {
            emit!("JMP .Lend_cmp_{}", id);
        }
    }

    emit!("LDI {} 0x01", reg_l);
    println!(".Lend_cmp_{id}:");
}

fn gen_fn_mb8(f: Function) {
    use self::IROp::{Imm, Mov, Add, AddImm, Sub, SubImm, AND, OR, XOR, Neg, SHL, SHR, Mul, MulImm, Div, Mod, EQ, NE, LT, LE, Load, Store, StoreArg, Bprel, Label, LabelAddr, Return, Jmp, If, Unless, Call, Nop, Kill};

    let ret_label = {
        let mut g = lock(&LABEL);
        let id = *g;
        *g += 1;
        format!(".Lret_{id}")
    };

    println!("{}:", f.name);

    for ir in f.ir {
        let lhs = ir.lhs.expect("missing lhs");
        let rhs = ir.rhs.unwrap_or(0);

        match ir.op {
            Imm => {
                emit!("LDI {} 0x{:02X}", REGS[lhs], rhs as u8);
            }
            Mov => {
                emit!("MOV {} {}", REGS[lhs], REGS[rhs]);
            }

            Add => {
                emit!("ADD {} {}", REGS[lhs], REGS[rhs]);
            }
            AddImm => {
                emit!("LDI R7 0x{:02X}", rhs as u8);
                emit!("ADD {} R7", REGS[lhs]);
            }
            Sub => {
                emit!("SUB {} {}", REGS[lhs], REGS[rhs]);
            }
            SubImm => {
                emit!("LDI R7 0x{:02X}", rhs as u8);
                emit!("SUB {} R7", REGS[lhs]);
            }
            AND => {
                emit!("AND {} {}", REGS[lhs], REGS[rhs]);
            }
            OR => {
                emit!("OR {} {}", REGS[lhs], REGS[rhs]);
            }
            XOR => {
                emit!("XOR {} {}", REGS[lhs], REGS[rhs]);
            }
            Neg => {
                emit!("ZERO R7");
                emit!("SUB R7 {}", REGS[lhs]);
                emit!("MOV {} R7", REGS[lhs]);
            }
            SHL => {
                emit!("MOV R7 {}", REGS[rhs]);
                emit!("SHL {} R7", REGS[lhs]);
            }
            SHR => {
                emit!("MOV R7 {}", REGS[rhs]);
                emit!("SHR {} R7", REGS[lhs]);
            }

            Mul => {
                emit!("MOV R7 {}", REGS[rhs]);
                emit!("MUL {} R7", REGS[lhs]);
            }
            MulImm => {
                emit!("LDI R7 0x{:02X}", rhs as u8);
                emit!("MUL {} R7", REGS[lhs]);
            }

            Div | Mod => {
                todo!()
            }

            EQ => emit_cmp_set_bool(lhs, rhs, "eq"),
            NE => emit_cmp_set_bool(lhs, rhs, "ne"),
            LT | LE | Nop | Kill => {}

            Load(_size) => {
                emit!("LDI R5 R6 0x0000");
                emit!("ADD R6 {}", REGS[lhs]);
                emit!("LD {} R5 R6", REGS[rhs]);
            }
            Store(_size) => {
                emit!("LDI R5 R6 0x0000");
                emit!("ADD R6 {}", REGS[lhs]);
                emit!("ST {} R5 R6", REGS[rhs]);
            }

            StoreArg(_size) => {
                todo!()
            }

            Bprel => {
                emit!("LDI {} {}", REGS[lhs], rhs as u8);
            }

            Label => {
                println!(".L{lhs}:");
            }

            LabelAddr(ref _name) => {}

            Return => {
                emit!("MOV R0 {}", REGS[lhs]);
                emit!("JMP {}", ret_label);
            }

            Jmp => {
                emit!("JMP .L{}", lhs);
            }

            If => {
                emit!("CMPI {} 0", REGS[lhs]);
                emit!("JNZR .L{}", rhs);
            }

            Unless => {
                emit!("CMPI {} 0", REGS[lhs]);
                emit!("JZR .L{}", rhs);
            }

            Call(ref name, nargs, ref args) => {
                for i in (0..nargs).rev() {
                    let reg = REGS[args[i]];
                    emit!("PUSH {}", reg);
                }

                emit!("CALL {}", name);

                for _ in 0..nargs {
                    emit!("POP R7");
                }

                emit!("MOV {} R0", REGS[lhs]);
            }

        }
    }

    println!("{ret_label}:");
    emit!("RET");
}

pub fn gen_mb8(globals: Vec<Var>, fns: Vec<Function>) {
    println!("#include \"../asm/cpu.asm\"");
    println!("#include \"../asm/ext.asm\"");
    println!();

    for var in globals {
        if let Scope::Global(data, len, is_extern) = var.scope {
            if is_extern {
                continue;
            }

            println!("{}:", var.name);
            if len == 0 {
                continue;
            }

            print!("\t#d8 ");
            let bytes = data.as_bytes();
            for i in 0..len {
                if i > 0 {
                    print!(", ");
                }
                let b = bytes.get(i).copied().unwrap_or(0);
                print!("0x{b:02X}");
            }
            println!();
        } else {
            unreachable!();
        }
    }

    for f in fns {
        gen_fn_mb8(f);
    }
}
