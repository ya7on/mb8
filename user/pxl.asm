#include "../asm/cpu.asm"
#include "../asm/ext.asm"

start:
    ; Set GPU mode to BITMAP (0x02) at 0xF000
    LDI R1 0x02
    ST [0xF000] R1

    ; IH:IL = 0xF001 (bitmap base)
    LDI IH 0xF0
    LDI IL 0x01

    ; Constants
    LDI R0 0x01     ; const 1
    LDI R3 0xFF     ; pixel byte

    ; Row counter (32 rows)
    LDI R4 0x20

.row_loop:
    ; Write 8 bytes for the row
    LDI R5 0x08
.col_loop:
    ST [IH:IL] R3
    INC16 IH IL
    SUB R5 R0
    CMPI R5 0x00
    JNZR [.col_loop]

    ; Small delay so rows appear gradually
    LDI R6 0x20
.delay_outer:
    LDI R5 0xFF
.delay_inner:
    SUB R5 R0
    CMPI R5 0x00
    JNZR [.delay_inner]
    SUB R6 R0
    CMPI R6 0x00
    JNZR [.delay_outer]

    ; Next row
    SUB R4 R0
    CMPI R4 0x00
    JNZR [.row_loop]

    LDI R0 0x0F
    CALL [0xE500]
