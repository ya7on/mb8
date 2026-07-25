.origin 0x1000

start:
    LDI R1, 0x09     ; SYS_FS_LIST
    LDI R2:R3, BUFFER
    CALL [0xE500]
    LDI R1:R2, BUFFER

    LDI R5, 0x00
    JMP [_file]
_end:
    LDI R1, 0x0F     ; SYS_EXIT
    CALL [0xE500]
_file:
    CMP R5, 0x10
    JZR [_end]

    LD R3, [R1:R2]
    CMP R3, 0x00     ; status
    JZR [_next_file]
_print_filename:
    LDI R7, 0x03
    ADD R2, R7       ; status - 0 -> start block + 1 -> size + 2 -> filename + 3

    PUSH R5
    MOV R3, R2
    MOV R2, R1
    LDI R1, 0x03     ; SYS_WRITELN
    CALL [0xE500]
    LDI R1, 0x02     ; SYS_WRITEL
    LDI R2, 0x0A
    CALL [0xE500]
    POP R5
_next_file:
    INC R5
    LDI R1:R2, BUFFER
    LDI R7, 0x10
    MUL R2, R7, R5

    JMP [_file]

.addr 0x1100
BUFFER:
.addr 0x1120
.addr 0x2000
