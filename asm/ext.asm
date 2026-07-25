; Extended (complex) opcodes for the virtual machine
; These are assembler-level macros built on top of existing instructions.
; They are NOT implemented by the VM itself and may modify registers or stack.
; Use with care.

#once

#include "isa.asm"

#ruledef mb8_isa_ext
{
    ; Load a 16-bit immediate into a register pair
    LDI { rh: register } { rl: register } { addr: u16 } => {
        hi = addr >> 8;
        lo = addr & 0xFF;

        asm {
            LDI {rh} {hi}
            LDI {rl} {lo}
        }
    }

    ; Call an absolute address
    CALL [{ addr: u16 }] => {
        hi = addr >> 8;
        lo = addr & 0xFF;
        asm {
            LDI IH {hi}
            LDI IL {lo}
            CALL [IH:IL]
        }
    }

    ; Jump to an absolute address
    JMP [{ addr: u16 }] => {
        hi = addr >> 8;
        lo = addr & 0xFF;
        asm {
            LDI IH {hi}
            LDI IL {lo}
            JMP [IH:IL]
        }
    }

    ; Jump to an absolute label using a relative offset
    JR [{ addr: u16 }] => {
        offset = addr - $ - 2
        $assert(offset <= 127)
        $assert(offset >= -128)
        0x31 @ offset`8
    }

    ; Jump if zero flag is set to an absolute label
    JZR [{ addr: u16 }] => {
        offset = addr - $ - 2
        $assert(offset <= 127)
        $assert(offset >= -128)
        0x32 @ offset`8
    }

    ; Jump if zero flag is not set to an absolute label
    JNZR [{ addr: u16 }] => {
        offset = addr - $ - 2
        $assert(offset <= 127)
        $assert(offset >= -128)
        0x33 @ offset`8
    }

    ; Jump if zero flag is not set to an absolute label
    JNCR [{ addr: u16 }] => {
        offset = addr - $ - 2
        $assert(offset <= 127)
        $assert(offset >= -128)
        0x35 @ offset`8
    }

    ; Clear register value
    ZERO { reg: register } => asm {
        LDI {reg} 0
    }

    ; Increment register value by one
    ; WARNING: This macro may modify the stack pointer.
    INC { reg: register } => asm {
        PUSH A
        LDI A 1
        ADD {reg} A
        POP A
    }

    ; Decrement register value by one
    ; WARNING: This macro may modify the stack pointer.
    DEC { reg: register } => asm {
        PUSH A
        LDI A 1
        SUB {reg} A
        POP A
    }

    ; Increment register pair as 16 bit value
    ; WARNING: This macro may modify the stack pointer.
    INC16 { hi: register }:{ lo: register } => asm {
        CMP {lo} 0xFF
        JZR [inc_hi]
        INC {lo}
        JR [end]
        inc_hi:
        LDI {lo} 0
        INC {hi}
        end:
        NOP
    }

    ; Negate register value
    ; WARNING: This macro may modify the stack pointer.
    NOT { reg: register } => asm {
        PUSH R7
        LDI R7 0xFF
        XOR {reg} R7
        POP R7
    }

    ; Compare register value with an immediate value and set flags accordingly
    ; WARNING: This macro may modify the stack pointer.
    CMP { reg: register } { val: u8 } => asm {
        PUSH A
        LDI A {val}
        CMP {reg} A
        POP A
    }

    ; Shift register value right by a given immediate value
    ; WARNING: This macro may modify the stack pointer.
    SHR { reg: register } { val: u8 } => asm {
        PUSH A
        LDI A {val}
        SHR {reg} A
        POP A
    }

    ; Shift register value left by a given immediate value
    ; WARNING: This macro may modify the stack pointer.
    SHL { reg: register } { val: u8 } => asm {
        PUSH A
        LDI A {val}
        SHL {reg} A
        POP A
    }

    ; Swap the values of two registers
    ; WARNING: This macro may modify the stack pointer.
    SWAP { reg1: register } { reg2: register } => asm {
        PUSH {reg1}
        MOV {reg1} {reg2}
        POP {reg2}
    }

    ; Multiply register `a` by register `b` and store result in `dst`
    MUL { dst: register } { a: register } { b: register } => asm {
        ZERO {dst}
        PUSH {b}
        iter:
            ADD {dst} {a}
            DEC {b}
            CMP {b} 0
            JNZR [iter]
        POP {b}
    }

    ST [{ addr: u16 }] { src: register } => {
        hi = addr >> 8
        lo = addr & 0xFF
        asm {
            LDI IH {hi}
            LDI IL {lo}
            ST [IH:IL] {src}
        }
    }

    LD { dst: register } [{ addr: u16 }] => {
        hi = addr >> 8
        lo = addr & 0xFF
        asm {
            LDI IH {hi}
            LDI IL {lo}
            LD {dst} [IH:IL]
        }
    }

    LD { dst: register } [{ hi: register }:{ lo: register } - { offset: u8 }] => asm {
        LDI R0 {offset}
        SUB {lo} R0
        JNCR [no_borrow]
        DEC {hi}
        no_borrow:
        LD {dst} [{hi}:{lo}]
    }
}
