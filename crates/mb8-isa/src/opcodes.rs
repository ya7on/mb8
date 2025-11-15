//! Opcodes for the MB8 ISA.
//! This module defines the opcodes used by the MB8 ISA.

use crate::registers::Register;

/// Syscall opcodes.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Syscall {
    Putc,
    Yield,
}

/// Full list of MB8 opcodes used in VM.
#[derive(Debug, PartialEq)]
pub enum Opcode {
    /* Control group */
    /// No operation. Instruction does nothing.
    Nop,
    /// Halt the VM.
    Halt,
    /// System call.
    Sys {
        syscall: Syscall,
        src: Register,
    },

    /* reg-reg opcodes */
    /// Move value from one register to another.
    Mov {
        dst: Register,
        src: Register,
    },
    /// Add `dst` and `src1` and store the result in `dst`.
    Add {
        dst: Register,
        src: Register,
    },
    /// Subtract `src` from `dst` and store the result in `dst`.
    Sub {
        dst: Register,
        src: Register,
    },
    /// Logical AND `dst` and `src1` and store the result in `dst`.
    And {
        dst: Register,
        src: Register,
    },
    /// Logical OR `dst` and `src1` and store the result in `dst`.
    Or {
        dst: Register,
        src: Register,
    },
    /// Logical XOR `dst` and `src1` and store the result in `dst`.
    Xor {
        dst: Register,
        src: Register,
    },
    /// Shift `dst` right by `src` bits and store the result in `dst`.
    Shr {
        dst: Register,
        src: Register,
    },
    /// Shift `dst` left by `src` bits and store the result in `dst`.
    Shl {
        dst: Register,
        src: Register,
    },

    /* Load */
    Ldi {
        dst: Register,
        value: u8,
    },

    /* Jump instructions */
    /// Jump to address `addr`.
    Jmp {
        addr: u16,
    },
    /// Jump to address `addr` if flag register has zero flag
    Jz {
        addr: u16,
    },
    /// Jump to address `addr` if flag register does not have zero flag
    Jnz {
        addr: u16,
    },
    /// Jump to address `addr` if flag register has carry flag
    Jc {
        addr: u16,
    },
    /// Jump to address `addr` if flag register does not have carry flag
    Jnc {
        addr: u16,
    },

    /* Stack instructions */
    /// Call subroutine at address `addr`.
    Call {
        addr: u16,
    },
    /// Return from subroutine.
    Ret,
    /// Push value from register `src` onto stack.
    Push {
        src: Register,
    },
    /// Pop value from stack into register `dst`.
    Pop {
        dst: Register,
    },

    /* Memory operations */
    /// Set memory index register `I` to `value`.
    LdiI {
        value: u16,
    },
    /// Load value from memory address stored in `I` register to `dst` register.
    Ld {
        dst: Register,
    },
    /// Store value in memory address stored in `I` register from `src` register.
    St {
        src: Register,
    },
    /// Increment memory address stored in `I` register by value stored in `src` register.
    IncI {
        src: Register,
    },
    /// Decrement memory address stored in `I` register by value stored in `src` register.
    DecI {
        src: Register,
    },

    /* Draw sprite */
    /// Draw sprite at position (x, y) with height `height`.
    Draw {
        x: Register,
        y: Register,
        height: u8,
    },
}
