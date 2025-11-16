# Instruction Set

- System instructions
  - [NOP](#nop)
  - [HALT](#halt)
  - [PUTC](#putc)
  - [YIELD](#yield)
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
  - [JZ](#jz)
  - [JNZ](#jnz)
  - [JC](#jc)
  - [JNC](#jnc)
- Stack instructions
  - [CALL](#call)
  - [RET](#ret)
  - [PUSH](#push)
  - [POP](#pop)
- Memory instructions
  - [LDI_I](#ldi_i)
  - [LD](#ld)
  - [ST](#st)
  - [INC_I](#inc_i)
  - [DEC_I](#dec_i)
  - [LDG](#ldg)
  - [STG](#stg)
- Draw instructions
  - [DRAW](#draw)
  
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

**Description**: No operation

**Example**:
```asm
NOP     ; No operation
```

---

## HALT

**Syntax**:
```asm
HALT [X]
```

**Args**:
- **X** - exit_code (optional)

**Encoding**:
```
0000 0001 XXXX XXXX
```

**Hex**: `0x01XX`

**Flags**: None

**Description**: Stop the execution of the program

**Example**:
```asm
HALT 0x1    ; Exit with code 1
HALT        ; This will never be executed because it is unreachable
```

---

## PUTC

**Syntax**:
```asm
PUTC rS
```

**Args**:
- **rS** - register which contains the character to be printed

**Encoding**:
```
0000 0010 0000 SSSS
```

**Hex**: `0x020S`

**Flags**: None

**Description**: Put character in register **rS** to the standard output

**Example**:
```asm
LDI R1 "A"  ; Put character 'A' in register R1
PUTC R1     ; Print character 'A'
```

---

## YIELD

**Syntax**:
```asm
YIELD rD
```

**Args**:
- **rD** - register which contains the bot ID

**Encoding**:
```
0000 0011 0000 DDDD
```

**Hex**: `0x030D`

**Flags**: None

**Description**: Give control to the bot with ID **rD**

**Example**:
```asm
LDI R1 0x1  ; Set bot ID to 1
YIELD R1    ; Give control to the bot with ID 1
HALT        ; This will be executed after the bot with ID 1 has finished executing
```

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
- **rD** - destination register
- **rS** - source register

**Encoding**:
```
0001 0000 DDDD SSSS
```

**Hex**: `0x10DS`

**Flags**: None

**Description**: Move the value from register **rS** to register **rD**

**Example**:
```asm
LDI R1 0x10     ; Load value 0x10 into register R1
MOV R2 R1       ; R2 = 0x10
```

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
- **rD** - destination register
- **rS** - source register

**Encoding**:
```
0001 0001 DDDD SSSS
```

**Hex**: `0x11DS`

**Flags**: Sets Z on zero, C on overflow (wrap past 0xFF), N mirrors bit 7 of the result

**Description**: Put the sum of the values in registers **rD** and **rS** into register **rD**

**Example**:
```asm
LDI R1 0x2
LDI R2 0x3
ADD R1 R2       ; R1 = 0x5
```

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
- **rD** - destination register
- **rS** - source register

**Encoding**:
```
0001 0010 DDDD SSSS
```

**Hex**: `0x12DS`

**Flags**: Sets Z on zero, C on borrow/overflow, N mirrors bit 7 of the result

**Description**: Put the difference of the values in registers **rD** and **rS** into register **rD**

**Example**:
```asm
LDI R1 0x3
LDI R2 0x1
SUB R1 R2       ; R1 = 0x2
```

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

**Args**:
- **rD** - destination register
- **rS** - source register

**Encoding**:
```
0001 0011 DDDD SSSS
```

**Hex**: `0x13DS`

**Flags**: Sets Z on zero; N mirrors bit 7; clears C

**Description**: Put the bitwise AND of the values in registers **rD** and **rS** into register **rD**

**Example**:
```asm
LDI R1 0b1111_1111
LDI R2 0b0000_1111
AND R1 R2       ; R1 = 0b0000_1111
```

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

**Args**:
- **rD** - destination register
- **rS** - source register

**Encoding**:
```
0001 0100 DDDD SSSS
```

**Hex**: `0x14DS`

**Flags**: Sets Z on zero; N mirrors bit 7; clears C

**Description**: Put the bitwise OR of the values in registers **rD** and **rS** into register **rD**

**Example**:
```asm
LDI R1 0b1111_0000
LDI R2 0b0000_1111
OR R1 R2       ; R1 = 0b1111_1111
```

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

**Args**:
- **rD** - destination register
- **rS** - source register

**Encoding**:
```
0001 0101 DDDD SSSS
```

**Hex**: `0x15DS`

**Flags**: Sets Z on zero; N mirrors bit 7; clears C

**Description**: Put the bitwise XOR of the values in registers **rD** and **rS** into register **rD**

**Example**:
```asm
LDI R1 0b1111_0000
LDI R2 0b1111_1111
XOR R1 R2       ; R1 = 0b0000_1111
```

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

**Args**:
- **rD** - destination register
- **rS** - source register

**Encoding**:
```
0001 0110 DDDD SSSS
```

**Hex**: `0x16DS`

**Flags**: Sets Z on zero, C when bits are shifted out (result > 0xFF), N mirrors bit 7

**Description**: Put the bitwise right shift of the value in register **rD** by the value in register **rS** into register **rD**

**Example**:
```asm
LDI R1 0b1111_0000
LDI R2 0x4
SHR R1 R2       ; R1 = 0b0000_1111
```

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

**Args**:
- **rD** - destination register
- **rS** - source register

**Encoding**:
```
0001 0111 DDDD SSSS
```

**Hex**: `0x17DS`

**Flags**: Sets Z on zero, C when bits are shifted out (result > 0xFF), N mirrors bit 7

**Description**: Put the bitwise left shift of the value in register **rD** by the value in register **rS** into register **rD**

**Example**:
```asm
LDI R1 0b0000_1111
LDI R2 0x4
SHL R1 R2       ; R1 = 0b1111_0000
```

# Register-immediate instructions

## LDI

**Syntax**:
```asm
LDI rD V
```

**Operation**:
```
rD = X
```

**Args**:
- **rD** - destination register
- **X** - immediate value

**Encoding**:
```
0010 DDDD XXXX XXXX
```

**Hex**: `0x2DXX`

**Flags**: None

**Description**: Load the immediate value **X** into register **rD**

**Example**:
```asm
; You can store decimal, hexadecimal, binary or string values into registers
LDI R1 0b0000_1111
LDI R2 0x0A
LDI R3 5
LDI R4 "H"
```

# Jump instructions

## JMP

**Syntax**:
```asm
JMP XXX
```

**Operation**:
```
PC = XXX
```

**Args**:
- **XXX** - address to jump to

**Encoding**:
```
0011 XXXX XXXX XXXX
```

**Hex**: `0x3XXX`

**Flags**: None

**Description**: Jump to the address **XXX**

**Example**:
```asm
; You can jump to specific addresses
JMP 0x100

; Or you can jump to labels
JMP label

label:
```

---

## JZ

**Syntax**:
```asm
JZ XXX
```

**Operation**:
```
if ZF == 1 then PC = XXX
```

**Args**:
- **XXX** - address to jump to

**Encoding**:
```
0100 XXXX XXXX XXXX
```

**Hex**: `0x4XXX`

**Flags**: None

**Description**: Jump to the address **XXX** if zero flag is set

**Example**:
```asm
LDI R1 2
LDI R2 2
SUB R1 R2   ; R1 = 0, Z flag is set
JZ label    ; Jump will be executed because Z flag is set

label:
```

---

## JNZ

**Syntax**:
```asm
JNZ XXX
```

**Operation**:
```
if ZF == 0 then PC = XXX
```

**Args**:
- **XXX** - address to jump to

**Encoding**:
```
0101 XXXX XXXX XXXX
```

**Hex**: `0x5XXX`

**Flags**: None

**Description**: Jump to the address **XXX** if zero flag is not set

**Example**:
```asm
LDI R1 2
LDI R2 2
SUB R1 R2   ; R1 = 0, Z flag is set
JNZ label   ; Jump will not be executed because Z flag is set

LDI R1 3
LDI R2 2
SUB R1 R2   ; R1 = 1, Z flag is not set
JNZ label   ; Jump will be executed because Z flag is not set

label:
```

---

## JC

**Syntax**:
```asm
JC XXX
```

**Operation**:
```
if CF == 1 then PC = XXX
```

**Args**:
- **XXX** - address to jump to

**Encoding**:
```
0110 XXXX XXXX XXXX
```

**Hex**: `0x6XXX`

**Flags**: None

**Description**: Jump to the address **XXX** if carry flag is set

**Example**:
```asm
LDI R1 255
LDI R2 2
ADD R1 R2   ; Register overflow occurred, Carry flag is set
JC label    ; Jump will be executed because Carry flag is set

LDI R1 3
LDI R2 2
SUB R1 R2   ; R1 = 1, Z flag is not set
JC label    ; Jump will not be executed because Carry flag is not set

label:
```

---

## JNC

**Syntax**:
```asm
JNC XXX
```

**Operation**:
```
if CF == 0 then PC = XXX
```

**Args**:
- **XXX** - address to jump to

**Encoding**:
```
0111 XXXX XXXX XXXX
```

**Hex**: `0x7XXX`

**Flags**: None

**Description**: Jump to the address **XXX** if carry flag is not set

**Example**:
```asm
LDI R1 255
LDI R2 2
ADD R1 R2   ; Register overflow occurred, Carry flag is set
JNC label   ; Jump will not be executed because Carry flag is set

LDI R1 3
LDI R2 2
SUB R1 R2   ; R1 = 1, Z flag is not set
JNC label   ; Jump will be executed because Carry flag is not set

label:
```

# Stack instructions

## CALL

**Syntax**:
```asm
CALL XXX
```

**Operation**:
```
push PC
PC = XXX
```

**Args**:
- **XXX** - address to call

**Encoding**:
```
1000 XXXX XXXX XXXX
```

**Hex**: `0x8XXX`

**Flags**: None

**Description**: Call the subroutine at the address **XXX**

**Example**:
```asm
CALL label  ; Call the subroutine at label

label:
    NOP
```

---

## RET

**Syntax**:
```asm
RET
```

**Operation**:
```
PC = pop()
```

**Args**: None

**Encoding**:
```
1001 0000 0000 0000
```

**Hex**: `0x9000`

**Flags**: None

**Description**: Return from the subroutine

**Example**:
```asm
CALL label  ; Call the subroutine at label
HALT

label:
    RET     ; Return from the subroutine
```

---

## PUSH

**Syntax**:
```asm
PUSH rS
```

**Operation**:
```
push rS
```

**Args**:
- **rS** - register to push

**Encoding**:
```
1001 0001 SSSS 0000
```

**Hex**: `0x91S0`

**Flags**: None

**Description**: Push the value of register **rS** onto the stack

**Example**:
```asm
LDI R1 255
PUSH R1   ; Push the value of R1 onto the stack
```

---

## POP

**Syntax**:
```asm
POP rD
```

**Operation**:
```
rD = pop()
```

**Args**:
- **rD** - register to pop into

**Encoding**:
```
1001 0010 DDDD 0000
```

**Hex**: `0x92D0`

**Flags**: None

**Description**: Pop the value from the stack into register **rD**

**Example**:
```asm
LDI R1 255
PUSH R1   ; Push the value of R1 onto the stack
POP R2    ; Pop the value from the stack into R2
```

# Memory instructions

## LDI_I

**Syntax**:
```asm
LDI_I XXX
```

**Operation**:
```
I = XXX
```

**Args**:
- **XXXX** - immediate value to load into I register

**Encoding**:
```
1010 XXXX XXXX XXXX
```

**Hex**: `0xAXXX`

**Flags**: None

**Description**: Load the immediate value **XXX** into the I register

**Example**:
```asm
LDI_I 0x123     ; I = 0x123
```

---

## LD

**Syntax**:
```asm
LD rD
```

**Operation**:
```
rD = mem[I]
```

**Args**:
- **rD** - register to load into

**Encoding**:
```
1011 0000 DDDD 0000
```

**Hex**: `0xB0D0`

**Flags**: None

**Description**: Load the value at memory address **I** into register **rD**

**Example**:
```asm
LDI_I 0x123     ; I = 0x123
LD R1           ; R1 = mem[0x123]
```

---

## ST

**Syntax**:
```asm
ST rS
```

**Operation**:
```
mem[I] = rS
```

**Args**:
- **rS** - register to store from

**Encoding**:
```
1011 0001 SSSS 0000
```

**Hex**: `0xB1S0`

**Flags**: None

**Description**: Store the value of register **rS** into memory address **I**

**Example**:
```asm
LDI_I 0x123     ; I = 0x123
LDI R1 0x11     ; R1 = 0x11
ST R1           ; mem[0x123] = R1 = 0x11
```

---

## INC_I

**Syntax**:
```asm
INC_I rS
```

**Operation**:
```
I = I + rS
```

**Args**:
- **rS** - register to add to I

**Encoding**:
```
1011 0010 SSSS 0000
```

**Hex**: `0xB2S0`

**Flags**: None

**Description**: Increment the value of **I** by the value of register **rS**

**Example**:
```asm
LDI_I 0x123     ; I = 0x123
LDI R1 0x2      ; R1 = 0x2
INC_I R1        ; I = I + R1 = 0x123 + 0x2 = 0x125
```

---

## DEC_I

**Syntax**:
```asm
DEC_I rS
```

**Operation**:
```
I = I - rS
```

**Args**:
- **rS** - register to subtract from I

**Encoding**:
```
1011 0011 SSSS 0000
```

**Hex**: `0xB3S0`

**Flags**: None

**Description**: Decrement the value of **I** by the value of register **rS**

**Example**:
```asm
LDI_I 0x123     ; I = 0x123
LDI R1 0x2      ; R1 = 0x2
DEC_I R1        ; I = I - R1 = 0x123 - 0x2 = 0x121
```

---

## LDG

**Syntax**:
```asm
LDG rD rB
```

**Operation**:
```
rD = bots[rB].shared_memory[I]
```

**Args**:
- **rD** - destination register to store the value from shared memory
- **rB** - register containing the bot ID

**Encoding**:
```
1011 0100 DDDD BBBB
```

**Hex**: `0xB4SB`

**Flags**: None

**Description**: Load the value of the shared memory of bot **rB** at address **I** into register **rD**

**Example**:
```asm
LDI_I 0x10      ; I = 0x10
LDI R1 0x11     ; R1 = 0x11
LDG R1 R2       ; R1 = bots[R2].shared_memory[0x10] = 0x11
```

---

## STG

**Syntax**:
```asm
STG rS rB
```

**Operation**:
```
bots[rB].shared_memory[I] = rS
```

**Args**:
- **rS** - source register containing the value to store
- **rB** - register containing the bot ID

**Encoding**:
```
1011 0101 SSSS BBBB
```

**Hex**: `0xB5SB`

**Flags**: None

**Description**: Store the value of register **rS** into the shared memory of bot **rB** at address **I**

**Example**:
```asm
LDI_I 0x10      ; I = 0x10
LDI R1 0x11     ; R1 = 0x11
STG R1 R2       ; bots[R2].shared_memory[0x10] = R1 = 0x11
```

# Draw instructions

## DRAW

**Syntax**:
```asm
DRAW rX rY H
```

**Operation**:
```
load_sprite_data(I)
draw_sprite(rX, rY, H)
```

**Args**:
- **rX** - x-coordinate register
- **rY** - y-coordinate register
- **H** - height value, count of rows to draw

**Encoding**:
```
1100 XXXX YYYY HHHH
```

**Hex**: `0xCXYH`

**Flags**: None

**Description**: Draw sprite at position (rX, rY) with height H. Sprite data is stored in memory at address I.

**Example**:
```asm
; If sprite is stored in RAM at address 0x123
LDI_I 0x123     ; I = 0x123
LDI R1 0x10     ; R1 = 0x10
LDI R2 0x20     ; R2 = 0x20
DRAW R1 R2 0x4  ; Draw sprite at position (0x10, 0x20) with height 4
```
