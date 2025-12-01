#include "../asm/cpu.asm"
#include "../asm/ext.asm"

; Hello World

start:
    ; SYS_WRITELN
    LDI R0 0x03
    LDI R1 R2 HELLO_WORLD
    CALL 0xE500

    ; SYS_EXIT
    LDI R0 0x0F
    CALL 0xE500

HELLO_WORLD:
    #d "Hello World!\n\0"
