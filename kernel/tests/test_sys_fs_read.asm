#include "../../asm/isa.asm"

#bankdef rom
{
    #addr 0xE000
    #size 0x1000
    #outp 0
    #fill
}

start:
    LDI R1 SYS_FS_READ
    LDI R2 R3 FILENAME
    LDI R4 0x00
    LDI R5 0x00
    CALL [K_SYSCALL_ENTRY]

    HALT

    #include "../syscalls.asm"


FILENAME:
    #d "file\0"
