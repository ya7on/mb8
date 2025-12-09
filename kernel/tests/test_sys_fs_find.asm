#include "../../asm/isa.asm"

#bankdef rom
{
    #addr 0xE000
    #size 0x1000
    #outp 0
    #fill
}

start:
    LDI R0 SYS_FS_FIND
    LDI R1 R2 FILENAME
    CALL [K_SYSCALL_ENTRY]

    HALT

    #include "../syscalls.asm"


FILENAME:
    #d "file\0"
