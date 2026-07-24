#include "../asm/cpu.asm"
#include "../asm/ext.asm"

; Minimal shell: read a line, exec filename at 0x0200 when Enter is pressed.

start:
    LDI R1 0x01
    ST [0xF000] R1
prompt:
    PUSH R2
    LDI R1 0x02       ; SYS_WRITE
    LDI R2 ">"
    CALL [0xE500]
    POP R2
    ZERO R2           ; idx = 0
read_key:
    PUSH R2
    LDI R1 0x04       ; SYS_WAIT_FOR_KEY
    CALL [0xE500]
    POP R2

    PUSH R2
    LDI R1 0x05       ; SYS_READ_KEY
    CALL [0xE500]
    POP R2

    CMP R1 "\n"
    JNZR [.post_read]
    JMP [exec_line]

.post_read:
    CMP R2 0x08
    JNZR [.print_char]
    JMP [read_key]

.print_char:
    ; store char at BUF + idx (page 0x02)
    LDI R4 0x02       ; hi
    ZERO R5
    MOV R5 R2         ; lo = idx (base lo = 0)
    ST [R4:R5] R1
    INC R2

    ; echo typed char
    PUSH R2
    MOV R2 R1
    LDI R1 0x02       ; SYS_WRITE
    CALL [0xE500]
    POP R2
    JMP [read_key]
exec_line:
    ; null-terminate at BUF+idx
    LDI R4 0x02
    MOV R5 R2
    LDI R1 0x00
    ST [R4:R5] R1

    ; newline then next prompt
    LDI R1 0x02
    LDI R2 "\n"
    CALL [0xE500]

    ; SYS_EXEC filename at BUF (0x0200)
    LDI R1 0x0E
    LDI R2 0x02       ; hi
    LDI R3 0x00       ; lo
    CALL [0xE500]

    LDI R1 0x03
    LDI R2 R3 NOT_FOUND
    CALL [0xE500]

    JMP [prompt]

#addr 0x1200
BUF:
    #d8 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00

NOT_FOUND:
    #d "Not found\n\0"
