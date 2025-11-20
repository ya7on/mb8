#include "../asm/isa.asm"

#bankdef rom
{
    #addr 0xE000
    #size 0x1000
    #outp 0
    #fill
}

start:
    LDI R0 0x01
    LDI R1 0xF0
    LDI R2 0x00
    ST R0 R1 R2

    LDI R2 0x01

    LDI R0 "m"
    ST R0 R1 R2
    LDI R0 "b"
    ST R0 R1 R2
    LDI R0 "8"
    ST R0 R1 R2
