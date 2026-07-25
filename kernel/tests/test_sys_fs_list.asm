.origin 0xE000

start:
    LDI R1, @SYS_FS_LIST
    LDI R2, 0x01
    LDI R3, 0x50
    CALL [K_SYSCALL_ENTRY]

    HALT

    .include "../syscalls.asm"

.addr 0xF000
