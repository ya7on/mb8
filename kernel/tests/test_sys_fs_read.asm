.origin 0xE000

start:
    LDI R1, @SYS_FS_READ
    LDI R2:R3, FILENAME
    LDI R4, 0x00
    LDI R5, 0x00
    CALL [K_SYSCALL_ENTRY]

    HALT

    .include "../syscalls.asm"


FILENAME:
    .ascii "file\0"

.addr 0xF000
