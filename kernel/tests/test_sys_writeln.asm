.origin 0xE000

start:
    LDI R1, @SYS_GPU_MODE
    LDI R2, 0x01
    CALL [K_SYSCALL_ENTRY]

    LDI R1, @SYS_WRITELN
    LDI R2:R3, HELLO_WORLD
    CALL [K_SYSCALL_ENTRY]

    HALT

    .include "../syscalls.asm"

HELLO_WORLD:
    .ascii "Hello, World!\0"

.addr 0xF000
