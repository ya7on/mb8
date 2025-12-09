#include "../../asm/isa.asm"

#bankdef rom
{
    #addr 0xE000
    #size 0x1000
    #outp 0
    #fill
}

start:
    LDI R0 SYS_GPU_MODE
    LDI R1 0x01
    CALL [K_SYSCALL_ENTRY]

    LDI R0 SYS_WRITELN
    LDI R1 R2 HELLO_WORLD
    CALL [K_SYSCALL_ENTRY]

    HALT

    #include "../syscalls.asm"

HELLO_WORLD:
    #d "Hello, World!\0"
