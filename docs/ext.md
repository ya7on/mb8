# Pseudo-instructions

Assembler-only helpers from `asm/ext.asm`. They rewrite into core opcodes and often use `R7` plus the stack as scratch.

- [ZERO](#zero)
- [INC](#inc)
- [DEC](#dec)
- [NOT](#not)
- [CMP](#cmp)
- [CMPI](#cmpi)
- [SHRI](#shri)
- [SHLI](#shli)
- [SWAP](#swap)

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
INC rD imm
```

**Expands to**:
```asm
PUSH R7
LDI R7 imm
ADD rD R7
POP R7
```

**Scratch**: uses `R7`, stack  
**Flags**: from `ADD` (Z/N/C)  
**Description**: Add an immediate to a register.

---

## DEC

**Syntax**:
```asm
DEC rD imm
```

**Expands to**:
```asm
PUSH R7
LDI R7 imm
SUB rD R7
POP R7
```

**Scratch**: uses `R7`, stack  
**Flags**: from `SUB` (Z/N/C)  
**Description**: Subtract an immediate from a register.

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

## CMP

**Syntax**:
```asm
CMP rA rB
```

**Expands to**:
```asm
PUSH rA
SUB rA rB
POP rA
```

**Scratch**: stack  
**Flags**: from `SUB` (Z/N/C)  
**Description**: Compare two registers; flags reflect `rA - rB`, operands restored.

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
