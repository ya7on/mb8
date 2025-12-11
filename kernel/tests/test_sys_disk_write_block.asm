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

    LDI R1 228
    LDI R2 0xF2
    LDI R3 0x02
    ST [R2:R3] R1

    LDI R0 SYS_DISK_WRITE_BLOCK
    CALL [K_SYSCALL_ENTRY]

    HALT

    #include "../syscalls.asm"
