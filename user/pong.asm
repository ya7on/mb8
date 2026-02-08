#include "../asm/cpu.asm"
#include "../asm/ext.asm"

; Minimal bitmap Pong
; Controls:
; - Left paddle:  W / S
; - Right paddle: O / L
; - Exit:         Esc

PADDLE_H = 0x04
PADDLE_MAX_Y = 0x1C

start:
    LDI R1 0x02
    ST [0xF000] R1

    LDI R1 0x0E
    ST [P1_Y] R1
    ST [P2_Y] R1

    LDI R1 0x20
    ST [BALL_X] R1

    LDI R1 0x10
    ST [BALL_Y] R1

    LDI R1 0x01
    ST [DX] R1
    ST [DY] R1

    LD R1 [P1_Y]
    ST [PREV_P1_Y] R1
    LD R1 [P2_Y]
    ST [PREV_P2_Y] R1
    LD R1 [BALL_X]
    ST [PREV_BALL_X] R1
    LD R1 [BALL_Y]
    ST [PREV_BALL_Y] R1

main_loop:
    CALL [read_input]
    CALL [update_ball]
    CALL [render]
    CALL [delay]
    JMP [main_loop]

read_input:
.loop:
    LDI R0 0x05
    CALL [0xE500]

    CMPI R0 0x00
    JNZR [.check_w]
    RET

.check_w:
    CMPI R0 "w"
    JNZR [.check_s]
    LD R1 [P1_Y]
    CMPI R1 0x00
    JNZR [.w_move]
    JMP [.done]
.w_move:
    DEC R1
    ST [P1_Y] R1
    JMP [.loop]

.check_s:
    CMPI R0 "s"
    JNZR [.check_o]
    LD R1 [P1_Y]
    CMPI R1 PADDLE_MAX_Y
    JNZR [.s_move]
    JMP [.done]
.s_move:
    INC R1
    ST [P1_Y] R1
    JMP [.loop]

.check_o:
    CMPI R0 "o"
    JNZR [.check_l]
    LD R1 [P2_Y]
    CMPI R1 0x00
    JNZR [.o_move]
    JMP [.done]
.o_move:
    DEC R1
    ST [P2_Y] R1
    JMP [.loop]

.check_l:
    CMPI R0 "l"
    JNZR [.check_esc]
    LD R1 [P2_Y]
    CMPI R1 PADDLE_MAX_Y
    JNZR [.l_move]
    JMP [.done]
.l_move:
    INC R1
    ST [P2_Y] R1
    JMP [.loop]

.check_esc:
    CMPI R0 0x1B
    JNZR [.done]
    LDI R0 0x0F
    CALL [0xE500]

.done:
    JMP [.loop]

update_ball:
    LD R1 [BALL_Y]
    LD R2 [DY]

    CMPI R1 0x00
    JNZR [.check_bottom]
    CMPI R2 0xFF
    JNZR [.check_bottom]
    LDI R2 0x01
    ST [DY] R2

.check_bottom:
    CMPI R1 0x1F
    JNZR [.apply_y]
    CMPI R2 0x01
    JNZR [.apply_y]
    LDI R2 0xFF
    ST [DY] R2

.apply_y:
    LD R1 [BALL_Y]
    LD R2 [DY]
    ADD R1 R2
    ST [BALL_Y] R1

    LD R1 [BALL_X]
    LD R2 [DX]

    CMPI R1 0x01
    JNZR [.check_right_paddle]
    CMPI R2 0xFF
    JNZR [.check_right_paddle]

    LD R3 [BALL_Y]
    LD R4 [P1_Y]
    CMP R3 R4
    JZR [.bounce_right]
    INC R4
    CMP R3 R4
    JZR [.bounce_right]
    INC R4
    CMP R3 R4
    JZR [.bounce_right]
    INC R4
    CMP R3 R4
    JZR [.bounce_right]
    JR [.check_right_paddle]

.bounce_right:
    LDI R2 0x01
    ST [DX] R2

.check_right_paddle:
    LD R1 [BALL_X]
    LD R2 [DX]

    CMPI R1 0x3E
    JNZR [.apply_x]
    CMPI R2 0x01
    JNZR [.apply_x]

    LD R3 [BALL_Y]
    LD R4 [P2_Y]
    CMP R3 R4
    JZR [.bounce_left]
    INC R4
    CMP R3 R4
    JZR [.bounce_left]
    INC R4
    CMP R3 R4
    JZR [.bounce_left]
    INC R4
    CMP R3 R4
    JZR [.bounce_left]
    JR [.apply_x]

.bounce_left:
    LDI R2 0xFF
    ST [DX] R2

.apply_x:
    LD R1 [BALL_X]
    LD R2 [DX]
    ADD R1 R2
    ST [BALL_X] R1

    CMPI R1 0x00
    JNZR [.check_miss_right]
    LDI R1 0x20
    ST [BALL_X] R1
    LDI R1 0x10
    ST [BALL_Y] R1
    LDI R1 0x01
    ST [DX] R1
    RET

.check_miss_right:
    CMPI R1 0x3F
    JNZR [.end]
    LDI R1 0x20
    ST [BALL_X] R1
    LDI R1 0x10
    ST [BALL_Y] R1
    LDI R1 0xFF
    ST [DX] R1

.end:
    RET

render:
    LD R1 [PREV_P1_Y]
    ZERO R2
    ZERO R3
    CALL [draw_paddle]

    LD R1 [PREV_P2_Y]
    LDI R2 0x07
    ZERO R3
    CALL [draw_paddle]

    LD R1 [PREV_BALL_Y]
    LD R2 [PREV_BALL_X]
    CALL [clear_ball]

    LD R1 [P1_Y]
    ZERO R2
    LDI R3 0x80
    CALL [draw_paddle]

    LD R1 [P2_Y]
    LDI R2 0x07
    LDI R3 0x01
    CALL [draw_paddle]

    LD R1 [BALL_Y]
    LD R2 [BALL_X]
    CALL [draw_ball]

    LD R1 [P1_Y]
    ST [PREV_P1_Y] R1
    LD R1 [P2_Y]
    ST [PREV_P2_Y] R1
    LD R1 [BALL_X]
    ST [PREV_BALL_X] R1
    LD R1 [BALL_Y]
    ST [PREV_BALL_Y] R1

    RET

draw_ball:
    PUSH R1
    PUSH R2

    MOV R4 R2
    SHRI R4 0x03

    MOV R5 R2
    LDI R0 0x07
    AND R5 R0

    LDI R6 R7 MASKS
    ADD R7 R5
    JNCR [.mask_no_carry]
    INC R6
.mask_no_carry:
    LD R3 [R6:R7]

    MOV R2 R4
    CALL [plot_byte]

    POP R2
    POP R1
    RET

clear_ball:
    PUSH R1
    PUSH R2
    MOV R4 R2
    SHRI R4 0x03
    MOV R2 R4
    ZERO R3
    CALL [plot_byte]
    POP R2
    POP R1
    RET

draw_paddle:
    LDI R4 PADDLE_H
.loop:
    PUSH R1
    PUSH R2
    PUSH R3
    PUSH R4
    CALL [plot_byte]
    POP R4
    POP R3
    POP R2
    POP R1

    INC R1
    DEC R4
    CMPI R4 0x00
    JNZR [.loop]
    RET

plot_byte:
    LDI R6 0xF0
    LDI R7 0x01

    MOV R4 R1
    SHLI R4 0x03
    ADD R7 R4
    JNCR [.no_carry_y]
    INC R6
.no_carry_y:

    ADD R7 R2
    JNCR [.no_carry_x]
    INC R6
.no_carry_x:

    ST [R6:R7] R3
    RET

clear_bitmap:
    LDI R6 0xF0
    LDI R7 0x01
    ZERO R3
    LDI R0 0x01
    LDI R4 0x20
.row_loop:
    LDI R5 0x08
.col_loop:
    ST [R6:R7] R3
    ADD R7 R0
    JNCR [.no_carry]
    ADD R6 R0
.no_carry:
    DEC R5
    CMPI R5 0x00
    JNZR [.col_loop]

    DEC R4
    CMPI R4 0x00
    JNZR [.row_loop]
    RET

delay:
    LDI R4 0x10
.outer:
    LDI R5 0xFF
.inner:
    DEC R5
    CMPI R5 0x00
    JNZR [.inner]
    DEC R4
    CMPI R4 0x00
    JNZR [.outer]
    RET

#addr 0x1F00
P1_Y:
    #d8 0x00
P2_Y:
    #d8 0x00
BALL_X:
    #d8 0x00
BALL_Y:
    #d8 0x00
DX:
    #d8 0x00
DY:
    #d8 0x00
PREV_P1_Y:
    #d8 0x00
PREV_P2_Y:
    #d8 0x00
PREV_BALL_X:
    #d8 0x00
PREV_BALL_Y:
    #d8 0x00
MASKS:
    #d8 0x80, 0x40, 0x20, 0x10, 0x08, 0x04, 0x02, 0x01
