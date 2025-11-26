use crate::gen_ir::{Function, IROp};
use crate::{Scope, Var, REGS_N};

use lazy_static::lazy_static;
use std::sync::Mutex;

const REGS: [&str; REGS_N] = ["R0", "R1", "R2", "R3", "R4", "R7", "R0"];

lazy_static! {
    static ref LABEL: Mutex<usize> = Mutex::new(0);
}

macro_rules! emit {
    ($fmt:expr) => (print!(concat!("\t", $fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!("\t", $fmt, "\n"), $($arg)*));
}

fn emit_cmp_set_bool(lhs: usize, rhs: usize, cond: &'static str) {
    let id = {
        let mut g = LABEL.lock().unwrap();
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
    println!(".Lend_cmp_{}:", id);
}

fn gen_fn_mb8(f: Function) {
    use self::IROp::*;

    let ret_label = {
        let mut g = LABEL.lock().unwrap();
        let id = *g;
        *g += 1;
        format!(".Lret_{}", id)
    };

    println!("{}:", f.name);

    for ir in f.ir {
        let lhs = ir.lhs.unwrap();
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

            Div | Mod => {}

            EQ => emit_cmp_set_bool(lhs, rhs, "eq"),
            NE => emit_cmp_set_bool(lhs, rhs, "ne"),
            LT | LE => {}

            Load(_size) => {
                emit!("ZERO R5");
                emit!("MOV R6 {}", REGS[rhs]);
                emit!("LD {} R5 R6", REGS[lhs]);
            }
            Store(_size) => {
                emit!("ZERO R5");
                emit!("MOV R6 {}", REGS[lhs]);
                emit!("ST {} R5 R6", REGS[rhs]);
            }

            StoreArg(_size) => {}

            Bprel => {}

            Label => {
                println!(".L{}:", lhs);
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

            Nop | Kill => {}
        }
    }

    println!("{}:", ret_label);
    emit!("RET");
}

pub fn gen_mb8(globals: Vec<Var>, fns: Vec<Function>) {
    println!("#include \"../asm/cpu.asm\"");
    println!("#include \"../asm/ext.asm\"");
    println!("#include \"../asm/std.asm\"");
    println!();
    println!("#bank rom");

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
                print!("0x{:02X}", b);
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
