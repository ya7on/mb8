.origin 0xE000

start:
    LDI R3, 0x02
    LDI R4, 0x00


    LDI R5, 0x10

rand_loop:

    LDI R1, @SYS_RAND
    CALL [K_SYSCALL_ENTRY] ; result returned in R1

    ST [R3:R4], R1


    LDI R6, 0x01
    ADD R4, R6

    LDI R6, 0x00
    CMP R4, R6
    JNZR [_skip_inc_high]

    LDI R6, 0x01
    ADD R3, R6

_skip_inc_high:

    LDI R6, 0x01
    SUB R5, R6

    LDI R6, 0x00
    CMP R5, R6
    JNZR [rand_loop]

    HALT

.include "../syscalls.asm"

.addr 0xF000
