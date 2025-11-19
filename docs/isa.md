# Instruction Set

- System instructions
  - [NOP](#nop)
  - [HALT](#halt)
  - [PUTC](#putc)
- Register-register instructions
  - [MOV](#mov)
  - [ADD](#add)
  - [SUB](#sub)
  - [AND](#and)
  - [OR](#or)
  - [XOR](#xor)
  - [SHR](#shr)
  - [SHL](#shl)
- Register-immediate instructions
  - [LDI](#ldi)
- Jump instructions
  - [JMP](#jmp)
  - [JR](#jr)
  - [JZR](#jzr)
  - [JNZR](#jnzr)
  - [JCR](#jcr)
  - [JNCR](#jncr)
- Stack instructions
  - [CALL](#call)
  - [RET](#ret)
  - [PUSH](#push)
  - [POP](#pop)
- Memory instructions
  - [LD](#ld)
  - [ST](#st)

# System instructions

## NOP

**Syntax**:
```asm
NOP
```

**Args**: None

**Encoding**:
```
0000 0000 0000 0000
```

**Hex**: `0x0000`

**Flags**: None

**Description**: Do nothing for one instruction cycle.

---

## HALT

**Syntax**:
```asm
HALT
```

**Args**: None

**Encoding**:
```
0000 0001 0000 0000
```

**Hex**: `0x0100`

**Flags**: None

**Description**: Stop the VM. Execution does not resume until a reset happens.

---

## PUTC

**Syntax**:
```asm
PUTC rS
```

**Args**:
- **rS** — register that holds the byte to print.

**Encoding**:
```
0000 0010 0000 SSSS
```

**Hex**: `0x020S`

**Flags**: None

**Description**: Write the value stored in **rS** to the console device.

---

# Register-register instructions

## MOV

**Syntax**:
```asm
MOV rD rS
```

**Operation**:
```
rD = rS
```

**Args**:
- **rD** — destination register.
- **rS** — source register.

**Encoding**:
```
0001 0000 DDDD SSSS
```

**Hex**: `0x10DS`

**Flags**: None

**Description**: Copy the value from **rS** into **rD**.

---

## ADD

**Syntax**:
```asm
ADD rD rS
```

**Operation**:
```
rD = rD + rS
```

**Args**:
- **rD** — destination register.
- **rS** — source register.

**Encoding**:
```
0001 0001 DDDD SSSS
```

**Hex**: `0x11DS`

**Flags**: Updates `Z` and `C`.

**Description**: Add **rS** to **rD**.

---

## SUB

**Syntax**:
```asm
SUB rD rS
```

**Operation**:
```
rD = rD - rS
```

**Args**:
- **rD** — destination register.
- **rS** — source register.

**Encoding**:
```
0001 0010 DDDD SSSS
```

**Hex**: `0x12DS`

**Flags**: Updates `Z` and `C`.

**Description**: Subtract **rS** from **rD**.

---

## AND

**Syntax**:
```asm
AND rD rS
```

**Operation**:
```
rD = rD & rS
```

**Encoding**:
```
0001 0011 DDDD SSSS
```

**Hex**: `0x13DS`

**Flags**: Updates `Z`.

**Description**: Bitwise AND.

---

## OR

**Syntax**:
```asm
OR rD rS
```

**Operation**:
```
rD = rD | rS
```

**Encoding**:
```
0001 0100 DDDD SSSS
```

**Hex**: `0x14DS`

**Flags**: Updates `Z`.

**Description**: Bitwise OR.

---

## XOR

**Syntax**:
```asm
XOR rD rS
```

**Operation**:
```
rD = rD ^ rS
```

**Encoding**:
```
0001 0101 DDDD SSSS
```

**Hex**: `0x15DS`

**Flags**: Updates `Z`.

**Description**: Bitwise XOR.

---

## SHR

**Syntax**:
```asm
SHR rD rS
```

**Operation**:
```
rD = rD >> rS
```

**Encoding**:
```
0001 0110 DDDD SSSS
```

**Hex**: `0x16DS`

**Flags**: Updates `Z` and `C`.

**Description**: Logical right shift by the amount in **rS**.

---

## SHL

**Syntax**:
```asm
SHL rD rS
```

**Operation**:
```
rD = rD << rS
```

**Encoding**:
```
0001 0111 DDDD SSSS
```

**Hex**: `0x17DS`

**Flags**: Updates `Z` and `C`.

**Description**: Logical left shift by the amount in **rS**.

---

# Register-immediate instructions

## LDI

**Syntax**:
```asm
LDI rD imm8
```

**Operation**:
```
rD = imm8
```

**Args**:
- **rD** — destination register.
- **imm8** — unsigned 8-bit immediate value.

**Encoding**:
```
0010 DDDD XXXX XXXX
```

**Hex**: `0x20DX`

**Flags**: None.

**Description**: Load an 8-bit immediate into **rD**.

# Jump instructions

## JMP

**Syntax**:
```asm
JMP rH rL
```

**Operation**:
```
PC = (rH << 8) | rL
```

**Args**:
- **rH** — register containing the high byte of the absolute address.
- **rL** — register containing the low byte of the absolute address.

**Encoding**:
```
0011 0000 HHHH LLLL
```

**Hex**: `0x30HL`

**Flags**: None.

**Description**: Absolute jump using two registers to form a 16-bit address.

---

## JR

**Syntax**:
```asm
JR off8
```

**Operation**:
```
PC = PC + sign_extend(off8)
```

**Args**:
- **off8** — signed 8-bit offset.

**Encoding**:
```
0011 0001 OOOO OOOO
```

**Hex**: `0x31OO`

**Flags**: None.

**Description**: Relative jump that always branches by the signed offset.

---

## JZR

**Syntax**:
```asm
JZR off8
```

**Operation**:
```
if Z == 1 { PC = PC + sign_extend(off8) }
```

**Encoding**:
```
0011 0010 OOOO OOOO
```

**Hex**: `0x32OO`

**Flags**: Reads `Z`.

**Description**: Relative jump taken only when the zero flag is set.

---

## JNZR

**Syntax**:
```asm
JNZR off8
```

**Operation**:
```
if Z == 0 { PC = PC + sign_extend(off8) }
```

**Encoding**:
```
0011 0011 OOOO OOOO
```

**Hex**: `0x33OO`

**Flags**: Reads `Z`.

**Description**: Relative jump taken only when the zero flag is clear.

---

## JCR

**Syntax**:
```asm
JCR off8
```

**Operation**:
```
if C == 1 { PC = PC + sign_extend(off8) }
```

**Encoding**:
```
0011 0100 OOOO OOOO
```

**Hex**: `0x34OO`

**Flags**: Reads `C`.

**Description**: Relative jump taken only when the carry flag is set.

---

## JNCR

**Syntax**:
```asm
JNCR off8
```

**Operation**:
```
if C == 0 { PC = PC + sign_extend(off8) }
```

**Encoding**:
```
0011 0101 OOOO OOOO
```

**Hex**: `0x35OO`

**Flags**: Reads `C`.

**Description**: Relative jump taken only when the carry flag is clear.

---

# Stack instructions

## CALL

**Syntax**:
```asm
CALL rH rL
```

**Operation**:
```
push(PC)
PC = (rH << 8) | rL
```

**Args**:
- **rH** — high byte register.
- **rL** — low byte register.

**Encoding**:
```
0100 0000 HHHH LLLL
```

**Hex**: `0x40HL`

**Flags**: None.

**Description**: Push the current `PC` onto the stack and jump to the absolute address formed by **rH**/**rL**.

---

## RET

**Syntax**:
```asm
RET
```

**Args**: None

**Encoding**:
```
0100 0001 0000 0000
```

**Hex**: `0x4100`

**Flags**: None.

**Description**: Pop the return address from the stack and jump to it.

---

## PUSH

**Syntax**:
```asm
PUSH rS
```

**Args**:
- **rS** — register to push.

**Encoding**:
```
0100 0010 0000 SSSS
```

**Hex**: `0x420S`

**Flags**: None.

**Description**: Decrement `SP`, store **rS** on the stack.

---

## POP

**Syntax**:
```asm
POP rD
```

**Args**:
- **rD** — destination register.

**Encoding**:
```
0100 0011 0000 DDDD
```

**Hex**: `0x430D`

**Flags**: None.

**Description**: Load a byte from the stack to **rD** and increment `SP`.

---

# Memory instructions

## LD

**Syntax**:
```asm
LD rD rH rL
```

**Operation**:
```
rD = MEM[(rH << 8) | rL]
```

**Args**:
- **rD** — destination register.
- **rH**/**rL** — registers holding the high/low bytes of the source address.

**Encoding**:
```
0101 DDDD HHHH LLLL
```

**Hex**: `0x5DHL`

**Flags**: None.

**Description**: Read one byte from RAM at the 16-bit address formed by **rH**/**rL** and place it in **rD**.

---

## ST

**Syntax**:
```asm
ST rS rH rL
```

**Operation**:
```
MEM[(rH << 8) | rL] = rS
```

**Args**:
- **rS** — source register.
- **rH**/**rL** — registers holding the high/low bytes of the destination address.

**Encoding**:
```
0110 SSSS HHHH LLLL
```

**Hex**: `0x6SHL`

**Flags**: None.

**Description**: Write one byte from **rS** to RAM at the 16-bit address composed from **rH**/**rL**.

---
