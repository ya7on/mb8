#include "../../asm/isa.asm"

#bankdef rom
{
    #addr 0xE000
    #size 0x1000
    #outp 0
    #fill
}

start:
    LDI R1 SYS_FS_DELETE
    LDI R2 R3 FILENAME
    CALL [K_SYSCALL_ENTRY]

    HALT

    #include "../syscalls.asm"


FILENAME:
    #d "file\0"
