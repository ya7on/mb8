#once

#include "syscalls.asm"

K_KERNEL_INIT:
    ; Initialize GPU
    LDI R1 SYS_GPU_MODE
    LDI R2 0x01
    CALL [K_SYSCALL_ENTRY]

    ; Write banner!
    LDI R1 SYS_WRITELN
    LDI R2 R3 MB8_BANNER
    CALL [K_SYSCALL_ENTRY]

    LDI R1 SYS_WRITELN
    LDI R2 R3 LOADING
    CALL [K_SYSCALL_ENTRY]

    JMP [0xE100]

#addr 0xE100
START_SHELL:
    LDI R1 SYS_EXEC
    LDI R2 R3 SHELL_BIN
    CALL [K_SYSCALL_ENTRY]

    LDI R1 SYS_WRITELN
    LDI R2 R3 SHELL_NOT_FOUND
    CALL [K_SYSCALL_ENTRY]

panic:
    JR [panic]

.error:
    HALT

MB8_BANNER:
    #d "MB8 kernel is starting...\n\0"

LOADING:
    #d "Type 'help' for more information\n\0"

SHELL_BIN:
    #d "sh\0"

SHELL_NOT_FOUND:
    #d "KERNEL PANIC: shell executable not found\n\0"
