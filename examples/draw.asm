; ===============
; Draw a rectangle
; ===============
; Simple program that draws a rectangle on the screen.

#include "../asm/cpu.asm"

start:
    LDI R0 0x8 ; X
    LDI R1 0x8 ; Y
    LDI_I .rect
    DRAW R0 R1 4
    HALT

.rect:
    #d 0b1111_1111
    #d 0b1111_1111
    #d 0b1111_1111
    #d 0b1111_1111
