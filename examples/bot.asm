#include "../asm/cpu.asm"

#bank rom

start:
    LDI R0 0x69
    YIELD
