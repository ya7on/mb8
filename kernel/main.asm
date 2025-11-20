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
    ; Initialize GPU
    LDI R0 SYS_GPU_MODE
    LDI R1 0x01
    CALL K_SYSCALL_ENTRY

    ; Write banner!
    LDI R0 SYS_WRITELN
    LDI R1 R2 MB8_BANNER
    CALL K_SYSCALL_ENTRY

    ; Write \n
    LDI R0 SYS_WRITE
    LDI R1 "\n"
    CALL K_SYSCALL_ENTRY
    ; Write >
    LDI R0 SYS_WRITE
    LDI R1 ">"
    CALL K_SYSCALL_ENTRY

.loop:
    JR .loop

#include "syscalls.asm"
#include "init.asm"

MB8_BANNER:
    #d "MB8 kernel is starting...\0"
