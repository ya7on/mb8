; Extended (complex) opcodes for the virtual machine
; These are assembler-level macros built on top of existing instructions.
; They are NOT implemented by the VM itself and may modify registers or stack.
; Use with care.

#ruledef mb8_isa_ext
{
    ; Clear register value
    ZERO { reg: register } => asm {
        LDI {reg} 0
    }

    ; Increment register value by a given immediate value
    ; WARNING: This macro may modify the stack pointer.
    INC { reg: register } { val: u8 } => asm {
        PUSH R7
        LDI R7 {val}
        ADD {reg} R7
        POP R7
    }

    ; Decrement register value by a given immediate value
    ; WARNING: This macro may modify the stack pointer.
    DEC { reg: register } { val: u8 } => asm {
        PUSH R7
        LDI R7 {val}
        SUB {reg} R7
        POP R7
    }

    ; Negate register value
    ; WARNING: This macro may modify the stack pointer.
    NOT { reg: register } => asm {
        PUSH R7
        LDI R7 0xFF
        XOR {reg} R7
        POP R7
    }

    ; Compare two registers and set flags accordingly
    ; WARNING: This macro may modify the stack pointer.
    CMP { reg1: register } { reg2: register } => asm {
        PUSH {reg1}
        SUB {reg1} {reg2}
        POP {reg1}
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
}
