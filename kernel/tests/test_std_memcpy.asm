.origin 0xE000

start:
    LDI R0, 0x00
    LDI R1, 0xFF
    LDI R2, 0x00
    LDI R3, 0x00
    LDI R4, 0x01
    LDI R5, 0x50

    MEMCPY [R4:R5], [R2:R3], R1

    HALT

.addr 0xF000
