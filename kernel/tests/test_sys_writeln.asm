#include "../../asm/isa.asm"

#bankdef rom
{
    #addr 0xE000
    #size 0x1000
    #outp 0
    #fill
}

start:
    LDI R1 SYS_GPU_MODE
    LDI R2 0x01
    CALL [K_SYSCALL_ENTRY]

    LDI R1 SYS_WRITELN
    LDI R2 R3 HELLO_WORLD
    CALL [K_SYSCALL_ENTRY]

    HALT

    #include "../syscalls.asm"

HELLO_WORLD:
    #d "Hello, World!\0"
