.origin 0xE000

start:
    LDI R1, @SYS_FS_FIND
    LDI R2:R3, FILENAME
    CALL [K_SYSCALL_ENTRY]

    HALT

    .include "../syscalls.asm"


FILENAME:
    .ascii "file\0"

.addr 0xF000
