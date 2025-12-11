#include "../asm/isa.asm"
#include "../asm/ext.asm"

#bankdef rom
{
    #addr 0xE000
    #size 0x1000
    #outp 0
    #fill
}

reset:
    JMP [K_KERNEL_INIT]
    HALT

#include "syscalls.asm"
#include "init.asm"
