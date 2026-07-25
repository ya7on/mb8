.origin 0x1000

start:
    ; Set GPU mode to BITMAP (0x02) at 0xF000
    LDI R1, 0x02
    ST [0xF000], R1

    ; IH:IL = 0xF001 (bitmap base)
    LDI IH, 0xF0
    LDI IL, 0x01

    ; Constants
    LDI A, 0x01 ; const 1
    LDI R3, 0xFF ; pixel byte

    ; Row counter (32 rows)
    LDI R4, 0x20

_row_loop:
    ; Write 8 bytes for the row
    LDI R5, 0x08
_col_loop:
    ST [IH:IL], R3
    INC16 IH:IL
    SUB R5, A
    CMP R5, 0x00
    JNZR [_col_loop]

    ; Small delay so rows appear gradually
    LDI R6, 0x20
_delay_outer:
    LDI R5, 0xFF
_delay_inner:
    SUB R5, A
    CMP R5, 0x00
    JNZR [_delay_inner]
    SUB R6, A
    CMP R6, 0x00
    JNZR [_delay_outer]

    ; Next row
    SUB R4, A
    CMP R4, 0x00
    JNZR [_row_loop]

    LDI R1, 0x0F
    CALL [0xE500]

.addr 0x2000
