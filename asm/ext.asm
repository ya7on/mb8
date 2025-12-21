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
            LDI R6 {hi}
            LDI R7 {lo}
            CALL [R6:R7]
        }
    }

    ; Jump to an absolute address
    JMP [{ addr: u16 }] => {
        hi = addr >> 8;
        lo = addr & 0xFF;
        asm {
            LDI R6 {hi}
            LDI R7 {lo}
            JMP [R6:R7]
        }
    }

    ; Jump to an absolute label using a relative offset
    JR [{ addr: u16 }] => {
        offset = addr - $ - 2
        assert(offset <= 127)
        assert(offset >= -128)
        0x31 @ offset`8
    }

    ; Jump if zero flag is set to an absolute label
    JZR [{ addr: u16 }] => {
        offset = addr - $ - 2
        assert(offset <= 127)
        assert(offset >= -128)
        0x32 @ offset`8
    }

    ; Jump if zero flag is not set to an absolute label
    JNZR [{ addr: u16 }] => {
        offset = addr - $ - 2
        assert(offset <= 127)
        assert(offset >= -128)
        0x33 @ offset`8
    }

    ; Clear register value
    ZERO { reg: register } => asm {
        LDI {reg} 0
    }

    ; Increment register value by one
    ; WARNING: This macro may modify the stack pointer.
    INC { reg: register } => asm {
        PUSH R7
        LDI R7 1
        ADD {reg} R7
        POP R7
    }

    ; Decrement register value by one
    ; WARNING: This macro may modify the stack pointer.
    DEC { reg: register } => asm {
        PUSH R7
        LDI R7 1
        SUB {reg} R7
        POP R7
    }

    ; Increment register pair as 16 bit value
    ; WARNING: This macro may modify the stack pointer.
    INC16 { hi: register } { lo: register } => asm {
        CMPI {lo} 0xFF
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
    CMPI { reg: register } { val: u8 } => asm {
        PUSH R7
        LDI R7 {val}
        SUB R7 {reg}
        POP R7
    }

    ; Shift register value right by a given immediate value
    ; WARNING: This macro may modify the stack pointer.
    SHRI { reg: register } { val: u8 } => asm {
        PUSH R7
        LDI R7 {val}
        SHR {reg} R7
        POP R7
    }

    ; Shift register value left by a given immediate value
    ; WARNING: This macro may modify the stack pointer.
    SHLI { reg: register } { val: u8 } => asm {
        PUSH R7
        LDI R7 {val}
        SHL {reg} R7
        POP R7
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
            CMPI {b} 0
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
}
