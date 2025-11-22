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

    ; Write \n
    LDI R0 SYS_WRITE
    LDI R1 "\n"
    CALL K_SYSCALL_ENTRY

    LDI R0 SYS_FS_FIND
    LDI R1 R2 SHELL_BIN
    CALL K_SYSCALL_ENTRY
    HALT

    LDI R0 SYS_FS_READ
    LDI R1 R2 SHELL_BIN
    LDI R2 0x10
    LDI R3 0x00
    CALL K_SYSCALL_ENTRY

    JMP 0x1000

    HALT

SHELL_BIN:
    #d "sh.bin"
MB8_BANNER:
    #d "MB8 kernel is starting...\0"
