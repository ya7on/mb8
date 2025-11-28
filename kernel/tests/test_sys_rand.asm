#include "../../asm/isa.asm"

#bankdef rom
{
    #addr 0xE000
    #size 0x1000
    #outp 0
    #fill
}

start:
    LDI R3 0x02
    LDI R4 0x00


    LDI R5 16

rand_loop:

    LDI R0 SYS_RAND
    CALL K_SYSCALL_ENTRY    ; result returned in R0

    ST R0 R3 R4


    LDI R6 1
    ADD R4 R6      

    LDI R6 0       
    CMP R4 R6
    JNZR .skip_inc_high

    LDI R6 1
    ADD R3 R6

.skip_inc_high:

    LDI R6 1
    SUB R5 R6

    LDI R6 0
    CMP R5 R6
    JNZR rand_loop

    HALT

#include "../syscalls.asm"
