#once

#include "syscalls.asm"

K_KERNEL_INIT:
    ; Initialize GPU
    LDI R0 SYS_GPU_MODE
    LDI R1 0x01
    CALL K_SYSCALL_ENTRY

    ; Write banner!
    LDI R0 SYS_WRITELN
    LDI R1 R2 MB8_BANNER
    CALL K_SYSCALL_ENTRY

    LDI R0 SYS_WRITELN
    LDI R1 R2 LOADING
    CALL K_SYSCALL_ENTRY

    LDI R0 SYS_FS_READ
    LDI R1 R2 SHELL_BIN
    LDI R3 0x10
    LDI R4 0x00
    CALL K_SYSCALL_ENTRY

    CMPI R0 0x00
    JNZR .error

    LDI R0 SYS_WRITELN
    LDI R1 R2 STARTING_SHELL
    CALL K_SYSCALL_ENTRY

    JMP 0x1000

.error:
    HALT

MB8_BANNER:
    #d "MB8 kernel is starting...\n\0"

LOADING:
    #d "Loading...\n\0"

STARTING_SHELL:
    #d "Starting shell...\n\0"

SHELL_BIN:
    #d "sh.bin\0"
