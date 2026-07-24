# Pseudo-instructions

Assembler-only helpers. They rewrite into core opcodes and often use `A` plus the stack as scratch.

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
- [CMP (immediate)](#cmp-immediate)
- [SHR (immediate)](#shr-immediate)
- [SHL (immediate)](#shl-immediate)
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
CALL [addr16]
```

**Expands to**:
```asm
LDI IH (addr16 >> 8)
LDI IL (addr16 & 0xFF)
CALL [IH:IL]
```

**Scratch**: uses `IH`, `IL`  
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
LDI IH (addr16 >> 8)
LDI IL (addr16 & 0xFF)
JMP [IH:IL]
```

**Scratch**: uses `IH`, `IL`  
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
PUSH A
LDI A 1
ADD rD A
POP A
```

**Scratch**: uses `A`, stack  
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
PUSH A
LDI A 1
SUB rD A
POP A
```

**Scratch**: uses `A`, stack  
**Flags**: from `SUB` (Z/N/C)  
**Description**: Decrement a register by one.

---

## INC16

**Syntax**:
```asm
INC16 rH:rL
```

**Expands to**:
```asm
CMP rL 0xFF
JZR inc_hi
INC rL
JR end
inc_hi:
LDI rL 0
INC rH
end:
NOP
```

**Scratch**: uses `A`, stack via `INC`  
**Flags**: from `CMP`, `INC` (Z/N/C)<br>
**Description**: Increment a 16-bit register pair in-place.

---

## NOT

**Syntax**:
```asm
NOT rD
```

**Expands to**:
```asm
PUSH A
LDI A 0xFF
XOR rD A
POP A
```

**Scratch**: uses `A`, stack  
**Flags**: from `XOR` (Z/N, clears C)  
**Description**: Bitwise invert a register.

---

## CMP (immediate)

**Syntax**:
```asm
CMP rD imm
```

**Expands to**:
```asm
PUSH A
LDI A imm
CMP rD A
POP A
```

**Scratch**: uses `A`, stack  
**Flags**: from `CMP` (Z/N/C)<br>
**Description**: Overloaded immediate form of `CMP`; flags reflect `rD - imm`.

---

## SHR (immediate)

**Syntax**:
```asm
SHR rD imm
```

**Expands to**:
```asm
PUSH A
LDI A imm
SHR rD A
POP A
```

**Scratch**: uses `A`, stack  
**Flags**: from `SHR` (Z/N/C)  
**Description**: Overloaded immediate form of `SHR`; shifts right by an immediate count.<br>

---

## SHL (immediate)

**Syntax**:
```asm
SHL rD imm
```

**Expands to**:
```asm
PUSH A
LDI A imm
SHL rD A
POP A
```

**Scratch**: uses `A`, stack  
**Flags**: from `SHL` (Z/N/C)  
**Description**: Overloaded immediate form of `SHL`; shifts left by an immediate count.<br>

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
    CMP rB 0
    JNZR iter
POP rB
```

**Scratch**: uses `A`, `rB`, stack  
**Flags**: from `ADD`, `DEC`, `CMP` (Z/N/C)<br>
**Description**: Unsigned multiply by repeated addition; destroys `rB` during the loop, restores it after.
