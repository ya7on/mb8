# Pseudo-instructions

Assembler-only helpers from `asm/ext.asm`. They rewrite into core opcodes and often use `R7` plus the stack as scratch.

- [LDI (16-bit)](#ldi-16-bit)
- [CALL (abs)](#call-abs)
- [JMP (abs)](#jmp-abs)
- [JR (abs)](#jr-abs)
- [JZR (abs)](#jzr-abs)
- [JNZR (abs)](#jnzr-abs)
- [ZERO](#zero)
- [INC](#inc)
- [DEC](#dec)
- [INC16](#inc16)
- [NOT](#not)
- [CMPI](#cmpi)
- [SHRI](#shri)
- [SHLI](#shli)
- [SWAP](#swap)
- [MUL](#mul)

---

## LDI (16-bit)

**Syntax**:
```asm
LDI rH rL imm16
```

**Expands to**:
```asm
LDI rH (imm16 >> 8)
LDI rL (imm16 & 0xFF)
```

**Scratch**: none  
**Flags**: none  
**Description**: Load a 16-bit immediate into two registers.

---

## CALL (abs)

**Syntax**:
```asm
CALL addr16
```

**Expands to**:
```asm
LDI R6 (addr16 >> 8)
LDI R7 (addr16 & 0xFF)
CALL R6 R7
```

**Scratch**: uses `R6`, `R7`  
**Flags**: none  
**Description**: Absolute subroutine call to a 16-bit address.

---

## JMP (abs)

**Syntax**:
```asm
JMP addr16
```

**Expands to**:
```asm
LDI R6 (addr16 >> 8)
LDI R7 (addr16 & 0xFF)
JMP R6 R7
```

**Scratch**: uses `R6`, `R7`  
**Flags**: none  
**Description**: Absolute jump to a 16-bit address.

---

## JR (abs)

**Syntax**:
```asm
JR label
```

**Expands to**: relative `JR` with the computed offset.

**Scratch**: none  
**Flags**: none  
**Description**: Jump to a label using a computed relative offset (assembler checks the range).

---

## JZR (abs)

**Syntax**:
```asm
JZR label
```

**Expands to**: relative `JZR` with the computed offset.

**Scratch**: none  
**Flags**: reads `Z`  
**Description**: Jump to a label when zero flag is set, using a computed offset.

---

## JNZR (abs)

**Syntax**:
```asm
JNZR label
```

**Expands to**: relative `JNZR` with the computed offset.

**Scratch**: none  
**Flags**: reads `Z`  
**Description**: Jump to a label when zero flag is clear, using a computed offset.

---

## ZERO

**Syntax**:
```asm
ZERO rD
```

**Expands to**:
```asm
LDI rD 0
```

**Scratch**: none  
**Flags**: none  
**Description**: Clear a register.

---

## INC

**Syntax**:
```asm
INC rD
```

**Expands to**:
```asm
PUSH R7
LDI R7 1
ADD rD R7
POP R7
```

**Scratch**: uses `R7`, stack  
**Flags**: from `ADD` (Z/N/C)  
**Description**: Increment a register by one.

---

## DEC

**Syntax**:
```asm
DEC rD
```

**Expands to**:
```asm
PUSH R7
LDI R7 1
SUB rD R7
POP R7
```

**Scratch**: uses `R7`, stack  
**Flags**: from `SUB` (Z/N/C)  
**Description**: Decrement a register by one.

---

## INC16

**Syntax**:
```asm
INC16 rH rL
```

**Expands to**:
```asm
CMPI rL 0xFF
JZR inc_hi
INC rL
JR end
inc_hi:
LDI rL 0
INC rH
end:
NOP
```

**Scratch**: stack via `INC`  
**Flags**: from `CMPI`, `INC` (Z/N/C)  
**Description**: Increment a 16-bit register pair in-place.

---

## NOT

**Syntax**:
```asm
NOT rD
```

**Expands to**:
```asm
PUSH R7
LDI R7 0xFF
XOR rD R7
POP R7
```

**Scratch**: uses `R7`, stack  
**Flags**: from `XOR` (Z/N, clears C)  
**Description**: Bitwise invert a register.

---

## CMPI

**Syntax**:
```asm
CMPI rD imm
```

**Expands to**:
```asm
PUSH R7
LDI R7 imm
SUB R7 rD
POP R7
```

**Scratch**: uses `R7`, stack  
**Flags**: from `SUB` (Z/N/C)  
**Description**: Compare a register with an immediate; flags reflect `imm - rD`.

---

## SHRI

**Syntax**:
```asm
SHRI rD imm
```

**Expands to**:
```asm
PUSH R7
LDI R7 imm
SHR rD R7
POP R7
```

**Scratch**: uses `R7`, stack  
**Flags**: from `SHR` (Z/N/C)  
**Description**: Shift right by an immediate count.

---

## SHLI

**Syntax**:
```asm
SHLI rD imm
```

**Expands to**:
```asm
PUSH R7
LDI R7 imm
SHL rD R7
POP R7
```

**Scratch**: uses `R7`, stack  
**Flags**: from `SHL` (Z/N/C)  
**Description**: Shift left by an immediate count.

---

## SWAP

**Syntax**:
```asm
SWAP rA rB
```

**Expands to**:
```asm
PUSH rA
MOV rA rB
POP rB
```

**Scratch**: stack  
**Flags**: none  
**Description**: Exchange the values of two registers.

---

## MUL

**Syntax**:
```asm
MUL rD rA rB
```

**Expands to**:
```asm
ZERO rD
PUSH rB
iter:
    ADD rD rA
    DEC rB
    CMPI rB 0
    JNZR iter
POP rB
```

**Scratch**: uses `rB`, stack  
**Flags**: from `ADD`, `DEC`, `CMPI` (Z/N/C)  
**Description**: Unsigned multiply by repeated addition; destroys `rB` during the loop, restores it after.
