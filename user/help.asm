#include "../asm/cpu.asm"
#include "../asm/ext.asm"

start:
    LDI R0 0x03
    LDI R1 R2 HELP_TEXT
    CALL 0xE500

    LDI R0 0x0F
    CALL 0xE500

HELP_TEXT:
    #d "MB8 - 8bit fantasy computer\n"
    #d "\n"
    #d "Commands:\n"
    #d "help.bin - Display help information\n"
    #d "ls.bin - List files\n"
    #d "hw.bin - Hello World\n"
    #d "exit.bin - Exit the system\n\0"
