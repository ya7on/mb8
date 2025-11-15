#include "../asm/cpu.asm"
#include "../asm/ext.asm"

start:
.init:
    ; Initialize sprites
    LDI R0 1
    ; M symbol
    LDI_I 0x100
    LDI R7 0b0000_0000
    ST R7
    INC_I R0
    LDI R7 0b0100_0100
    ST R7
    INC_I R0
    LDI R7 0b0111_1100
    ST R7
    INC_I R0
    LDI R7 0b0101_0100
    ST R7
    INC_I R0
    LDI R7 0b0100_0100
    ST R7
    INC_I R0
    LDI R7 0b1110_1110
    ST R7
    INC_I R0
    ; B symbol
    LDI_I 0x200
    LDI R7 0b00000000
    ST R7
    INC_I R0
    LDI R7 0b01111100
    ST R7
    INC_I R0
    LDI R7 0b01000010
    ST R7
    INC_I R0
    LDI R7 0b01011100
    ST R7
    INC_I R0
    LDI R7 0b01000010
    ST R7
    INC_I R0
    LDI R7 0b01111100
    ST R7
    INC_I R0
    ; 8 symbol
    LDI_I 0x300
    LDI R7 0b00000000
    ST R7
    INC_I R0
    LDI R7 0b00011000
    ST R7
    INC_I R0
    LDI R7 0b01100110
    ST R7
    INC_I R0
    LDI R7 0b00111100
    ST R7
    INC_I R0
    LDI R7 0b01100110
    ST R7
    INC_I R0
    LDI R7 0b00011000
    ST R7
    INC_I R0
.draw:
    ; Draw M symbol
    LDI R0 22
    LDI R1 13
    LDI_I 0x100
    DRAW R0 R1 6
    ; Draw B symbol
    LDI R0 29
    LDI R1 13
    LDI_I 0x200
    DRAW R0 R1 6
    ; Draw 8 symbol
    LDI R0 36
    LDI R1 13
    LDI_I 0x300
    DRAW R0 R1 6
.loop:
    ; Loop forever
    JMP .loop
