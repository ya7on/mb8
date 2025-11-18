#include "../asm/isa.asm"

#bankdef rom
{
    #addr 0xE000
    #size 0x1000
    #outp 0
    #fill
}

start:
    NOP
    LDI R0 0x12
    LDI R1 0x34
    MOV R0 R1
    HALT
