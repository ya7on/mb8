#include "../../asm/isa.asm"

#bankdef rom
{
    #addr 0xE000
    #size 0x1000
    #outp 0
    #fill
}

start:
    LDI R0 SYS_DISK_SET_BLOCK
    LDI R1 0x01
    CALL [K_SYSCALL_ENTRY]

    LDI R0 SYS_DISK_READ_BLOCK
    CALL [K_SYSCALL_ENTRY]

    HALT

    #include "../syscalls.asm"
