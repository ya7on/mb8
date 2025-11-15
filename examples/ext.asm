; ===============
; Extended features
; ===============
; These opcodes aren't implemented in the virtual machine.
; They are just macroses on top of the existing instructions.

#include "../asm/cpu.asm"
#include "../asm/ext.asm" ; Extended (complex) opcodes

start:
    ZERO R0 ; Clearing value in the register
    INC R0 1 ; Incrementing value in the register
    DEC R0 1 ; Decrementing value in the register
    NOT R0 ; Inverting value in the register
    CMP R0 R1 ; Comparing values of two registers
    CMPI R0 5 ; Comparing value of register with immediate value
    SHRI R0 2 ; Shifting value of register right by 2 bits
    SHLI R0 3 ; Shifting value of register left by 3 bits
    SWAP R0 R1 ; Swapping values of two registers
    JMPR 0x1 ; Relative jump to address PC + 0x1
