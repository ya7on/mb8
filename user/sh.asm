#include "../asm/cpu.asm"
#include "../kernel/syscalls.asm"

start:
    ; Write >
    LDI R0 SYS_WRITE
    LDI R1 ">"
    CALL K_SYSCALL_ENTRY
.input:
    JR .input

    HALT
