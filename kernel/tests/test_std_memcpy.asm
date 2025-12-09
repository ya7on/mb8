#include "../../asm/isa.asm"
#include "../../asm/std.asm"

#bankdef rom
{
    #addr 0xE000
    #size 0x1000
    #outp 0
    #fill
}

start:
    LDI R0 0
    LDI R1 255
    LDI R2 0x00
    LDI R3 0x00
    LDI R4 0x01
    LDI R5 0x50

    MEMCPY [R4:R5] [R2:R3] R1

    HALT
