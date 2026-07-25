.origin 0xE000

start:
    LDI R1, @SYS_DISK_SET_BLOCK
    LDI R2, 0x01
    CALL [K_SYSCALL_ENTRY]

    LDI R1, 0xE4
    LDI R2, 0xF2
    LDI R3, 0x02
    ST [R2:R3], R1

    LDI R1, @SYS_DISK_WRITE_BLOCK
    CALL [K_SYSCALL_ENTRY]

    HALT

    .include "../syscalls.asm"

.addr 0xF000
