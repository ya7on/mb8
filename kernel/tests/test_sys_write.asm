.origin 0xE000

start:
    LDI R1, @SYS_GPU_MODE
    LDI R2, 0x01
    CALL [K_SYSCALL_ENTRY]

    LDI R1, @SYS_WRITE
    LDI R2, 0x31
    CALL [K_SYSCALL_ENTRY]

    LDI R1, @SYS_WRITE
    LDI R2, 0x32
    CALL [K_SYSCALL_ENTRY]

    LDI R1, @SYS_WRITE
    LDI R2, 0x33
    CALL [K_SYSCALL_ENTRY]

    HALT

    .include "../syscalls.asm"

.addr 0xF000
