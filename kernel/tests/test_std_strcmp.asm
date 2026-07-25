.origin 0xE000

start:
    LDI R0, 0x00
    LDI R1, 0x00
    LDI R2, 0x00
    LDI R3, 0x00
    LDI R4, 0x00
    LDI R5, 0x14

    STRCMP R0, R1, R2, R3, R4, R5

    HALT

.addr 0xF000
