.origin 0x1000

; Minimal bitmap Pong
; Controls:
; - Left paddle:  W / S
; - Right paddle: O / L
; - Exit:         Esc

.const @PADDLE_H, 0x04
.const @PADDLE_MAX_Y, 0x1C

start:
    LDI R1, 0x02
    ST [0xF000], R1

    LDI R1, 0x0E
    ST [P1_Y], R1
    ST [P2_Y], R1

    LDI R1, 0x20
    ST [BALL_X], R1

    LDI R1, 0x10
    ST [BALL_Y], R1

    LDI R1, 0x01
    ST [DX], R1
    ST [DY], R1

    LD R1, [P1_Y]
    ST [PREV_P1_Y], R1
    LD R1, [P2_Y]
    ST [PREV_P2_Y], R1
    LD R1, [BALL_X]
    ST [PREV_BALL_X], R1
    LD R1, [BALL_Y]
    ST [PREV_BALL_Y], R1

main_loop:
    CALL [read_input]
    CALL [update_ball]
    CALL [render]
    CALL [delay]
    JMP [main_loop]

read_input:
_loop:
    LDI R1, 0x05
    CALL [0xE500]

    CMP R1, 0x00
    JNZR [_check_w]
    RET

_check_w:
    CMP R1, 0x77
    JNZR [_check_s]
    LD R1, [P1_Y]
    CMP R1, 0x00
    JNZR [_w_move]
    JMP [_done]
_w_move:
    DEC R1
    ST [P1_Y], R1
    JMP [_loop]

_check_s:
    CMP R1, 0x73
    JNZR [_check_o]
    LD R1, [P1_Y]
    CMP R1, @PADDLE_MAX_Y
    JNZR [_s_move]
    JMP [_done]
_s_move:
    INC R1
    ST [P1_Y], R1
    JMP [_loop]

_check_o:
    CMP R1, 0x6F
    JNZR [_check_l]
    LD R1, [P2_Y]
    CMP R1, 0x00
    JNZR [_o_move]
    JMP [_done]
_o_move:
    DEC R1
    ST [P2_Y], R1
    JMP [_loop]

_check_l:
    CMP R1, 0x6C
    JNZR [_check_esc]
    LD R1, [P2_Y]
    CMP R1, @PADDLE_MAX_Y
    JNZR [_l_move]
    JMP [_done]
_l_move:
    INC R1
    ST [P2_Y], R1
    JMP [_loop]

_check_esc:
    CMP R1, 0x1B
    JNZR [_done]
    LDI R1, 0x0F
    CALL [0xE500]

_done:
    JMP [_loop]

update_ball:
    LD R1, [BALL_Y]
    LD R2, [DY]

    CMP R1, 0x00
    JNZR [_check_bottom]
    CMP R2, 0xFF
    JNZR [_check_bottom]
    LDI R2, 0x01
    ST [DY], R2

_check_bottom:
    CMP R1, 0x1F
    JNZR [_apply_y]
    CMP R2, 0x01
    JNZR [_apply_y]
    LDI R2, 0xFF
    ST [DY], R2

_apply_y:
    LD R1, [BALL_Y]
    LD R2, [DY]
    ADD R1, R2
    ST [BALL_Y], R1

    LD R1, [BALL_X]
    LD R2, [DX]

    CMP R1, 0x01
    JNZR [_check_right_paddle]
    CMP R2, 0xFF
    JNZR [_check_right_paddle]

    LD R3, [BALL_Y]
    LD R4, [P1_Y]
    CMP R3, R4
    JZR [_bounce_right]
    INC R4
    CMP R3, R4
    JZR [_bounce_right]
    INC R4
    CMP R3, R4
    JZR [_bounce_right]
    INC R4
    CMP R3, R4
    JZR [_bounce_right]
    JR [_check_right_paddle]

_bounce_right:
    LDI R2, 0x01
    ST [DX], R2

_check_right_paddle:
    LD R1, [BALL_X]
    LD R2, [DX]

    CMP R1, 0x3E
    JNZR [_apply_x]
    CMP R2, 0x01
    JNZR [_apply_x]

    LD R3, [BALL_Y]
    LD R4, [P2_Y]
    CMP R3, R4
    JZR [_bounce_left]
    INC R4
    CMP R3, R4
    JZR [_bounce_left]
    INC R4
    CMP R3, R4
    JZR [_bounce_left]
    INC R4
    CMP R3, R4
    JZR [_bounce_left]
    JR [_apply_x]

_bounce_left:
    LDI R2, 0xFF
    ST [DX], R2

_apply_x:
    LD R1, [BALL_X]
    LD R2, [DX]
    ADD R1, R2
    ST [BALL_X], R1

    CMP R1, 0x00
    JNZR [_check_miss_right]
    LDI R1, 0x20
    ST [BALL_X], R1
    LDI R1, 0x10
    ST [BALL_Y], R1
    LDI R1, 0x01
    ST [DX], R1
    RET

_check_miss_right:
    CMP R1, 0x3F
    JNZR [_end]
    LDI R1, 0x20
    ST [BALL_X], R1
    LDI R1, 0x10
    ST [BALL_Y], R1
    LDI R1, 0xFF
    ST [DX], R1

_end:
    RET

render:
    LD R1, [PREV_P1_Y]
    ZERO R2
    ZERO R3
    CALL [draw_paddle]

    LD R1, [PREV_P2_Y]
    LDI R2, 0x07
    ZERO R3
    CALL [draw_paddle]

    LD R1, [PREV_BALL_Y]
    LD R2, [PREV_BALL_X]
    CALL [clear_ball]

    LD R1, [P1_Y]
    ZERO R2
    LDI R3, 0x80
    CALL [draw_paddle]

    LD R1, [P2_Y]
    LDI R2, 0x07
    LDI R3, 0x01
    CALL [draw_paddle]

    LD R1, [BALL_Y]
    LD R2, [BALL_X]
    CALL [draw_ball]

    LD R1, [P1_Y]
    ST [PREV_P1_Y], R1
    LD R1, [P2_Y]
    ST [PREV_P2_Y], R1
    LD R1, [BALL_X]
    ST [PREV_BALL_X], R1
    LD R1, [BALL_Y]
    ST [PREV_BALL_Y], R1

    RET

draw_ball:
    PUSH R1
    PUSH R2

    MOV R4, R2
    SHR R4, 0x03

    MOV R5, R2
    LDI R0, 0x07
    AND R5, R0

    LDI R6:R7, MASKS
    ADD R7, R5
    JNCR [_mask_no_carry]
    INC R6
_mask_no_carry:
    LD R3, [R6:R7]

    MOV R2, R4
    CALL [plot_byte]

    POP R2
    POP R1
    RET

clear_ball:
    PUSH R1
    PUSH R2
    MOV R4, R2
    SHR R4, 0x03
    MOV R2, R4
    ZERO R3
    CALL [plot_byte]
    POP R2
    POP R1
    RET

draw_paddle:
    LDI R4, @PADDLE_H
_loop:
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
    CMP R4, 0x00
    JNZR [_loop]
    RET

plot_byte:
    LDI R6, 0xF0
    LDI R7, 0x01

    MOV R4, R1
    SHL R4, 0x03
    ADD R7, R4
    JNCR [_no_carry_y]
    INC R6
_no_carry_y:

    ADD R7, R2
    JNCR [_no_carry_x]
    INC R6
_no_carry_x:

    ST [R6:R7], R3
    RET

clear_bitmap:
    LDI R6, 0xF0
    LDI R7, 0x01
    ZERO R3
    LDI R0, 0x01
    LDI R4, 0x20
_row_loop:
    LDI R5, 0x08
_col_loop:
    ST [R6:R7], R3
    ADD R7, R0
    JNCR [_no_carry]
    ADD R6, R0
_no_carry:
    DEC R5
    CMP R5, 0x00
    JNZR [_col_loop]

    DEC R4
    CMP R4, 0x00
    JNZR [_row_loop]
    RET

delay:
    LDI R4, 0x10
_outer:
    LDI R5, 0xFF
_inner:
    DEC R5
    CMP R5, 0x00
    JNZR [_inner]
    DEC R4
    CMP R4, 0x00
    JNZR [_outer]
    RET

.addr 0x1F00
P1_Y:
    .data 0x00
P2_Y:
    .data 0x00
BALL_X:
    .data 0x00
BALL_Y:
    .data 0x00
DX:
    .data 0x00
DY:
    .data 0x00
PREV_P1_Y:
    .data 0x00
PREV_P2_Y:
    .data 0x00
PREV_BALL_X:
    .data 0x00
PREV_BALL_Y:
    .data 0x00
MASKS:
    .data 0x80, 0x40, 0x20, 0x10, 0x08, 0x04, 0x02, 0x01

.addr 0x2000
