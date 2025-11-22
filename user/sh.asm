#include "../asm/cpu.asm"
#include "../asm/ext.asm"

start:
    ; Write >
    LDI R0 0x02
    LDI R1 ">"
    CALL 0xE500

.input:
    ; Wait for keypress
    LDI R0 0x04
    CALL 0xE500

    LDI R0 0x05
    CALL 0xE500

    MOV R1 R0
    LDI R0 0x02
    CALL 0xE500

    JR .input

    HALT
