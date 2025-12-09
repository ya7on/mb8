#once

#include "ext.asm"

#ruledef mb8_std
{
    MEMCPY [{ dsthi: register }:{ dstlo: register}] [{ srchi: register }:{ srclo: register}] { len: register } => asm {
        PUSH A
        ZERO A
        loop:
            PUSH A
            LD A [{srchi}:{srclo}]
            ST [{dsthi}:{dstlo}] A
            POP A

            CMP A {len}
            JZR [end]
            INC A

            INC16 {srchi} {srclo}
            INC16 {dsthi} {dstlo}

            JR [loop]
        end:
            POP A
    }

    ; Compare two zero-terminated strings, returns 0 in `i` if equal, 1 otherwise
    STRCMP { i: register } { j: register } { srchi: register } { srclo: register } { dsthi: register } { dstlo: register } => asm {
        loop:
        LD {i} [{srchi}:{srclo}]
        LD {j} [{dsthi}:{dstlo}]

        CMP {i} {j}
        JNZR [error]

        CMPI {j} "\0"
        JZR [success]

        INC16 {srchi} {srclo}
        INC16 {dsthi} {dstlo}

        JMP [loop]

        error:
        LDI {i} 1
        JR [end]
        success:
        LDI {i} 0
        end:
    }
}
