#once

#include "ext.asm"

#ruledef mb8_std
{
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

    STRCMP { i: register } { j: register } { dsthi: register } { dstlo: register } { srchi: register } { srclo: register } => asm {
        LD {i} {dsthi} {dstlo}
        LD {j} {srchi} {srclo}

        CMPI {i} "\0"
        JZR .end_of_str
        CMP {i} {j}
        JNZR .error

        end_of_str:
        CMPI {j} "\0"
        JZR .success
        error:
        LDI {i} 1
        JR .end
        success:
        LDI {i} 0
        end:
    }
}
