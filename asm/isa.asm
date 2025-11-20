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
    SP => 0xD
    PC => 0xE
    F => 0xF
}

#ruledef mb8_isa
{
    NOP  => 0x0000
    HALT => 0x0100
    HALT { code: u8 } => 0x01 @ code
    MOV { dst: register } { src: register } => 0x10 @ dst @ src
    ADD { dst: register } { src: register } => 0x11 @ dst @ src
    SUB { dst: register } { src: register } => 0x12 @ dst @ src
    AND { dst: register } { src: register } => 0x13 @ dst @ src
    OR { dst: register } { src: register } => 0x14 @ dst @ src
    XOR { dst: register } { src: register } => 0x15 @ dst @ src
    SHR { dst: register } { src: register } => 0x16 @ dst @ src
    SHL { dst: register } { src: register } => 0x17 @ dst @ src
    LDI { dst: register } { value: u8 } => 0x2 @ dst @ value
    JMP { hi: register } { lo: register } => 0x30 @ hi @ lo
    JR { offset: u8 } => 0x31 @ offset
    JZR { offset: u8 } => 0x32 @ offset
    JNZR { offset: u8 } => 0x33 @ offset
    JCR { offset: u8 } => 0x34 @ offset
    JNCR { offset: u8 } => 0x35 @ offset
    CALL { hi: register } { lo: register } => 0x40 @ hi @ lo
    RET => 0x4100
    PUSH { src: register } => 0x42 @ src @ 0x0
    POP { dst: register } => 0x43 @ dst @ 0x0
    LD { dst: register } { hi: register } { lo: register } => 0x5 @ dst @ hi @ lo
    ST { dst: register } { hi: register } { lo: register } => 0x6 @ dst @ hi @ lo
}
