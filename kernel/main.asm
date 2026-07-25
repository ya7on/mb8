.origin 0xE000

reset:
    JMP [K_KERNEL_INIT]
    HALT

.include "init.asm"
.include "syscalls.asm"

K_KERNEL_INIT:
    ; Initialize GPU
    LDI R1, @SYS_GPU_MODE
    LDI R2, 0x01
    CALL [K_SYSCALL_ENTRY]

    ; Write banner!
    LDI R1, @SYS_WRITELN
    LDI R2:R3, MB8_BANNER
    CALL [K_SYSCALL_ENTRY]

    LDI R1, @SYS_WRITELN
    LDI R2:R3, LOADING
    CALL [K_SYSCALL_ENTRY]

    JMP [0xE100]

.addr 0xF000
