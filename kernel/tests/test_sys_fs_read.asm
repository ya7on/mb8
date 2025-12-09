#include "../../asm/isa.asm"

#bankdef rom
{
    #addr 0xE000
    #size 0x1000
    #outp 0
    #fill
}

start:
    LDI R0 SYS_FS_READ
    LDI R1 R2 FILENAME
    LDI R3 0x00
    LDI R4 0x00
    CALL [K_SYSCALL_ENTRY]

    HALT

    #include "../syscalls.asm"


FILENAME:
    #d "file\0"
