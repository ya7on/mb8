#once

#include "ext.asm"

#ruledef mb8_std
{
    ; Copy `len` bytes from `srchi:srclo` to `dsthi:dstlo` using index register `i`
    MEMCPY { i: register } { len: register } { srchi: register } { srclo: register } { dsthi: register } { dstlo: register } => asm {
        loop:
        PUSH {i}
        LD {i} {srchi} {srclo}
        ST {i} {dsthi} {dstlo}
        POP {i}

        CMP {i} {len}
        JZR end
        INC {i}

        INC16 {srchi} {srclo}
        INC16 {dsthi} {dstlo}

        JR loop
        end:
    }

    ; Compare two zero-terminated strings, returns 0 in `i` if equal, 1 otherwise
    STRCMP { i: register } { j: register } { srchi: register } { srclo: register } { dsthi: register } { dstlo: register } => asm {
        loop:
        LD {i} {srchi} {srclo}
        LD {j} {dsthi} {dstlo}

        CMP {i} {j}
        JNZR error

        CMPI {j} "\0"
        JZR success

        INC16 {srchi} {srclo}
        INC16 {dsthi} {dstlo}

        JMP loop

        error:
        LDI {i} 1
        JR end
        success:
        LDI {i} 0
        end:
    }
}
