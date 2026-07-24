#include "../../asm/isa.asm"

#bankdef rom
{
    #addr 0xE000
    #size 0x1000
    #outp 0
    #fill
}

start:
    LDI R1 SYS_DISK_SET_BLOCK
    LDI R2 0x01
    CALL [K_SYSCALL_ENTRY]
    HALT

    #include "../syscalls.asm"
