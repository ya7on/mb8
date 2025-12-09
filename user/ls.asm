#include "../asm/cpu.asm"
#include "../asm/ext.asm"


start:
    LDI R0 0x09     ; SYS_FS_LIST
    LDI R1 R2 BUFFER
    CALL [0xE500]
    LDI R1 R2 BUFFER

    LDI R5 0x00
    JMP [.file]
.end:
    LDI R0 0x0F     ; SYS_EXIT
    CALL [0xE500]
.file:
    CMPI R5 0x10
    JZR [.end]

    LD R3 [R1:R2]
    CMPI R3 0x00    ; status
    JZR [.next_file]
.print_filename:
    LDI R7 0x03
    ADD R2 R7       ; status - 0 -> start block + 1 -> size + 2 -> filename + 3

    PUSH R5
    LDI R0 0x03     ; SYS_WRITELN
    CALL [0xE500]
    LDI R0 0x02     ; SYS_WRITEL
    LDI R1 "\n"
    CALL [0xE500]
    POP R5
.next_file:
    INC R5
    LDI R1 R2 BUFFER
    LDI R7 0x10
    MUL R2 R7 R5

    JMP [.file]

#addr 0x1100
BUFFER:
    #d256 0
