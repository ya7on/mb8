#include "../../asm/isa.asm"

#bankdef rom
{
    #addr 0xE000
    #size 0x1000
    #outp 0
    #fill
}

start:
    LDI R0 SYS_FS_LIST
    LDI R1 0x01
    LDI R2 0x50
    CALL [K_SYSCALL_ENTRY]

    HALT

    #include "../syscalls.asm"
