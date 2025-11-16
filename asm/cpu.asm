#bankdef rom
{
    #addr 0x0000
    #size 0x1000
    #outp 0
    #fill
}

#bankdef ram
{
    #addr 0x0000
    #size 0x0E00
    #outp 8 * 0x1000
    #fill
}

#subruledef register
{
    R0 => 0x0
    R1 => 0x1
    R2 => 0x2
    R3 => 0x3
    R4 => 0x4
    R5 => 0x5
    R6 => 0x6
    R7 => 0x7
    I => 0xC
    SP => 0xD
    PC => 0xE
    F => 0xF
}

#ruledef mb8_isa
{
    NOP  => 0x0000
    HALT => 0x0100
    HALT { code: u8 } => 0x01 @ code
    SYS { subcode: u4 } { reg: register } => 0x02 @ subcode @ reg
    PUTC { src: register } => 0x020 @ src
    YIELD => 0x0210
    YIELD { src: register } => 0x021 @ src
    MOV { dst: register } { src: register } => 0x10 @ dst @ src
    ADD { dst: register } { src: register } => 0x11 @ dst @ src
    SUB { dst: register } { src: register } => 0x12 @ dst @ src
    AND { dst: register } { src: register } => 0x13 @ dst @ src
    OR { dst: register } { src: register } => 0x14 @ dst @ src
    XOR { dst: register } { src: register } => 0x15 @ dst @ src
    SHR { dst: register } { src: register } => 0x16 @ dst @ src
    SHL { dst: register } { src: register } => 0x17 @ dst @ src
    LDI { dst: register } { value: u8 } => 0x2 @ dst @ value
    JMP { addr: u12 } => 0x3 @ addr
    JZ { addr: u12 } => 0x4 @ addr
    JNZ { addr: u12 } => 0x5 @ addr
    JC { addr: u12 } => 0x6 @ addr
    JNC { addr: u12 } => 0x7 @ addr
    CALL { label: u12 } => 0x8 @ label
    RET => 0x9000
    PUSH { src: register } => 0x91 @ src @ 0x0
    POP { dst: register } => 0x92 @ dst @ 0x0
    LDI_I { addr: u12 } => 0xA @ addr
    LD { dst: register } => 0xB0 @ dst @ 0x0
    ST { src: register } => 0xB1 @ src @ 0x0
    INC_I { src: register } => 0xB2 @ src @ 0x0
    DEC_I { src: register } => 0xB3 @ src @ 0x0
    LDG { dst: register } { bot: register } => 0xB4 @ dst @ bot
    STG { src: register } { bot: register } => 0xB5 @ src @ bot
    DRAW { x: register } { y: register } { height: u4 } => 0xC @ x @ y @ height
}
