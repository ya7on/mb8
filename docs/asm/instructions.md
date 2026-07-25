# Assembler instruction reference

This page lists every instruction form accepted by the MB8 assembler. Mnemonics are ordered
alphabetically. Forms within a mnemonic are ordered as core, immediate, label, and other pseudo
forms.

## Conventions

- `rD`, `rS`, `rA`, and `rB` are 8-bit registers.
- `rH:rL` is a pair of 8-bit registers containing a 16-bit value.
- `imm8` and `imm16` are hexadecimal immediate values.
- `off8` is the encoded 8-bit value of a signed relative offset.
- `label` is a label defined in the assembled program.
- An immediate operand may also be supplied through a constant such as `@NAME`.
- Square brackets denote an address or a memory operand and are part of the syntax.
- `hi(value)` and `lo(value)` are compiler expressions that select the high and low bytes.
- `rel8(target)` is the signed offset from the following instruction to `target`.
- `{id}` is the unique source-instruction index used in compiler-generated labels.
- Registers read and written list actual accesses made by the emitted instructions, including
  temporary accesses. Scratch registers state whether those temporary values are preserved.

## Mnemonics

[ADD](#add) · [AND](#and) · [CALL](#call) · [CMP](#cmp) · [DEC](#dec) · [HALT](#halt) ·
[INC](#inc) · [INC16](#inc16) · [JCR](#jcr) · [JMP](#jmp) · [JNCR](#jncr) · [JNZR](#jnzr) ·
[JR](#jr) · [JZR](#jzr) · [LD](#ld) · [LDI](#ldi) · [MEMCPY](#memcpy) · [MOV](#mov) ·
[MUL](#mul) · [NOP](#nop) · [NOT](#not) · [OR](#or) · [POP](#pop) · [PUSH](#push) ·
[RET](#ret) · [SHL](#shl) · [SHR](#shr) · [ST](#st) · [STRCMP](#strcmp) · [SUB](#sub) ·
[SWAP](#swap) · [SYS](#sys) · [XOR](#xor) · [ZERO](#zero)

## ADD

### ADD rD, rS

**Kind:** [Core](../hard/isa.md#add).  
**Operands:** `rD` is the destination and left operand; `rS` is the right operand.  
**Compiles to:**

```asm
ADD rD, rS
```

**Operation:** Adds `rS` to `rD`, stores the wrapped 8-bit result in `rD`, and updates arithmetic
flags.  
**Registers read:** `rD`, `rS`.  
**Registers written:** `rD`, `F`.  
**Scratch registers:** None.  
**Stack:** None.  
**Flags:** Reads: None; writes: `Z` when the result is zero, `N` from result bit 7, and `C` on
unsigned overflow.

## AND

### AND rD, rS

**Kind:** [Core](../hard/isa.md#and).  
**Operands:** `rD` is the destination and left operand; `rS` is the right operand.  
**Compiles to:**

```asm
AND rD, rS
```

**Operation:** Stores the bitwise AND of `rD` and `rS` in `rD`.  
**Registers read:** `rD`, `rS`.  
**Registers written:** `rD`, `F`.  
**Scratch registers:** None.  
**Stack:** None.  
**Flags:** Reads: None; writes: `Z` when the result is zero, `N` from result bit 7, and clears `C`.

## CALL

### CALL [rH:rL]

**Kind:** [Core](../hard/isa.md#call).  
**Operands:** `rH:rL` contains the absolute destination address.  
**Compiles to:**

```asm
CALL [rH:rL]
```

**Operation:** Pushes the address of the following instruction and sets `PC` to the address in
`rH:rL`.  
**Registers read:** `rH`, `rL`, `PC`, `SPH`, `SPL`.  
**Registers written:** `PC`, `SPH`, `SPL`.  
**Scratch registers:** None.  
**Stack:** Pushes a two-byte return address; net stack-pointer change is -2 bytes.  
**Flags:** Reads: None; writes: None.

### CALL [imm16]

**Kind:** Pseudo.  
**Operands:** `imm16` is the absolute destination address.  
**Compiles to:**

```asm
LDI IH, hi(imm16)
LDI IL, lo(imm16)
CALL [IH:IL]
```

**Operation:** Loads `imm16` into `IH:IL`, pushes the address of the following instruction, and
sets `PC` to `imm16`.  
**Registers read:** `PC`, `SPH`, `SPL`, `IH`, `IL`.  
**Registers written:** `IH`, `IL`, `PC`, `SPH`, `SPL`.  
**Scratch registers:** `IH`, `IL` are clobbered.  
**Stack:** Pushes a two-byte return address; net stack-pointer change is -2 bytes.  
**Flags:** Reads: None; writes: None.

### CALL [label]

**Kind:** Pseudo.  
**Operands:** `label` identifies the absolute destination address.  
**Compiles to:**

```asm
LDI IH, hi(label)
LDI IL, lo(label)
CALL [IH:IL]
```

**Operation:** Loads the address of `label` into `IH:IL`, pushes the address of the following
instruction, and transfers control to `label`.  
**Registers read:** `PC`, `SPH`, `SPL`, `IH`, `IL`.  
**Registers written:** `IH`, `IL`, `PC`, `SPH`, `SPL`.  
**Scratch registers:** `IH`, `IL` are clobbered.  
**Stack:** Pushes a two-byte return address; net stack-pointer change is -2 bytes.  
**Flags:** Reads: None; writes: None.

## CMP

### CMP rD, rS

**Kind:** [Core](../hard/isa.md#cmp).  
**Operands:** `rD` is the left operand; `rS` is the right operand.  
**Compiles to:**

```asm
CMP rD, rS
```

**Operation:** Computes `rD - rS` for flags without modifying either operand.  
**Registers read:** `rD`, `rS`.  
**Registers written:** `F`.  
**Scratch registers:** None.  
**Stack:** None.  
**Flags:** Reads: None; writes: `Z` when the difference is zero, `N` from difference bit 7, and
`C` on unsigned underflow.

### CMP rD, imm8

**Kind:** Pseudo.  
**Operands:** `rD` is the left operand; `imm8` is the right operand.  
**Compiles to:**

```asm
PUSH A
LDI A, imm8
CMP rD, A
POP A
```

**Operation:** Compares `rD` with `imm8` while preserving the original value of `A`.  
**Registers read:** `rD`, `A`, `SPH`, `SPL`.  
**Registers written:** `A`, `F`, `SPH`, `SPL`.  
**Scratch registers:** `A` is temporarily modified and restored.  
**Stack:** Uses one temporary byte; maximum depth is 1 byte and net stack-pointer change is 0.  
**Flags:** Reads: None; writes: `Z` when `rD == imm8`, `N` from difference bit 7, and `C` when
`rD < imm8`.

## DEC

### DEC rD

**Kind:** Pseudo.  
**Operands:** `rD` is the register to decrement.  
**Compiles to:**

```asm
PUSH A
LDI A, 0x01
SUB rD, A
POP A
```

**Operation:** Subtracts one from `rD` with 8-bit wrapping while preserving `A`.  
**Registers read:** `rD`, `A`, `SPH`, `SPL`.  
**Registers written:** `rD`, `A`, `F`, `SPH`, `SPL`.  
**Scratch registers:** `A` is temporarily modified and restored.  
**Stack:** Uses one temporary byte; maximum depth is 1 byte and net stack-pointer change is 0.  
**Flags:** Reads: None; writes: `Z` when the result is zero, `N` from result bit 7, and `C` when
the decrement underflows.

## HALT

### HALT

**Kind:** [Core](../hard/isa.md#halt).  
**Operands:** None.  
**Compiles to:**

```asm
HALT
```

**Operation:** Stops VM execution with an encoded low byte of `0x00`.  
**Registers read:** None.  
**Registers written:** None.  
**Scratch registers:** None.  
**Stack:** None.  
**Flags:** Reads: None; writes: None.

### HALT imm8

**Kind:** [Core encoded form](../hard/isa.md#halt).  
**Operands:** `imm8` is placed in the low byte of the instruction.  
**Compiles to:**

```asm
HALT imm8
```

**Operation:** Stops VM execution. The current processor ignores the encoded low byte after
decoding.  
**Registers read:** None.  
**Registers written:** None.  
**Scratch registers:** None.  
**Stack:** None.  
**Flags:** Reads: None; writes: None.

## INC

### INC rD

**Kind:** Pseudo.  
**Operands:** `rD` is the register to increment.  
**Compiles to:**

```asm
PUSH A
LDI A, 0x01
ADD rD, A
POP A
```

**Operation:** Adds one to `rD` with 8-bit wrapping while preserving `A`.  
**Registers read:** `rD`, `A`, `SPH`, `SPL`.  
**Registers written:** `rD`, `A`, `F`, `SPH`, `SPL`.  
**Scratch registers:** `A` is temporarily modified and restored.  
**Stack:** Uses one temporary byte; maximum depth is 1 byte and net stack-pointer change is 0.  
**Flags:** Reads: None; writes: `Z` when the result is zero, `N` from result bit 7, and `C` when
the increment overflows.

## INC16

### INC16 rH:rL

**Kind:** Pseudo.  
**Operands:** `rH:rL` contains the 16-bit value to increment.  
**Compiles to:**

```asm
PUSH A
LDI A, 0xFF
CMP rL, A
POP A
JZR rel8(__mb8_inc16_hi_{id})
PUSH A
LDI A, 0x01
ADD rL, A
POP A
JR rel8(__mb8_inc16_end_{id})
__mb8_inc16_hi_{id}:
LDI rL, 0x00
PUSH A
LDI A, 0x01
ADD rH, A
POP A
__mb8_inc16_end_{id}:
NOP
```

**Operation:** Increments `rH:rL`; increments only `rL` unless it was `0xFF`, in which case it
sets `rL` to zero and increments `rH`. `A` is preserved.  
**Registers read:** `rH`, `rL`, `A`, `F`, `PC`, `SPH`, `SPL`.  
**Registers written:** `rH` conditionally, `rL`, `A`, `F`, `PC`, `SPH`, `SPL`.  
**Scratch registers:** `A` is temporarily modified and restored.  
**Stack:** Uses one temporary byte at a time; maximum depth is 1 byte and net stack-pointer change
is 0.  
**Flags:** Reads: `Z` for the generated branch; writes: `Z`, `N`, and `C` from the byte incremented
by the final `ADD`.

## JCR

### JCR off8

**Kind:** [Core](../hard/isa.md#jcr).  
**Operands:** `off8` is the encoded signed offset from the following instruction.  
**Compiles to:**

```asm
JCR off8
```

**Operation:** Adds the signed offset to `PC` when `C` is set; otherwise continues at the following
instruction.  
**Registers read:** `F`, `PC`.  
**Registers written:** `PC` when the branch is taken.  
**Scratch registers:** None.  
**Stack:** None.  
**Flags:** Reads: `C`; writes: None.

### JCR [imm16]

**Kind:** Pseudo.  
**Operands:** `imm16` is an absolute target address converted to a relative offset.  
**Compiles to:**

```asm
JCR rel8(imm16)
```

**Operation:** Branches to `imm16` when `C` is set. The target must fit a signed 8-bit offset from
the following instruction.  
**Registers read:** `F`, `PC`.  
**Registers written:** `PC` when the branch is taken.  
**Scratch registers:** None.  
**Stack:** None.  
**Flags:** Reads: `C`; writes: None.

### JCR [label]

**Kind:** Pseudo.  
**Operands:** `label` is converted to a relative offset.  
**Compiles to:**

```asm
JCR rel8(label)
```

**Operation:** Branches to `label` when `C` is set. The target must fit a signed 8-bit offset from
the following instruction.  
**Registers read:** `F`, `PC`.  
**Registers written:** `PC` when the branch is taken.  
**Scratch registers:** None.  
**Stack:** None.  
**Flags:** Reads: `C`; writes: None.

## JMP

### JMP [rH:rL]

**Kind:** [Core](../hard/isa.md#jmp).  
**Operands:** `rH:rL` contains the absolute destination address.  
**Compiles to:**

```asm
JMP [rH:rL]
```

**Operation:** Sets `PC` to the 16-bit address in `rH:rL`.  
**Registers read:** `rH`, `rL`.  
**Registers written:** `PC`.  
**Scratch registers:** None.  
**Stack:** None.  
**Flags:** Reads: None; writes: None.

### JMP [imm16]

**Kind:** Pseudo.  
**Operands:** `imm16` is the absolute destination address.  
**Compiles to:**

```asm
LDI IH, hi(imm16)
LDI IL, lo(imm16)
JMP [IH:IL]
```

**Operation:** Loads `imm16` into `IH:IL` and transfers control to it.  
**Registers read:** `IH`, `IL`.  
**Registers written:** `IH`, `IL`, `PC`.  
**Scratch registers:** `IH`, `IL` are clobbered.  
**Stack:** None.  
**Flags:** Reads: None; writes: None.

### JMP [label]

**Kind:** Pseudo.  
**Operands:** `label` identifies the absolute destination address.  
**Compiles to:**

```asm
LDI IH, hi(label)
LDI IL, lo(label)
JMP [IH:IL]
```

**Operation:** Loads the address of `label` into `IH:IL` and transfers control to it.  
**Registers read:** `IH`, `IL`.  
**Registers written:** `IH`, `IL`, `PC`.  
**Scratch registers:** `IH`, `IL` are clobbered.  
**Stack:** None.  
**Flags:** Reads: None; writes: None.

## JNCR

### JNCR off8

**Kind:** [Core](../hard/isa.md#jncr).  
**Operands:** `off8` is the encoded signed offset from the following instruction.  
**Compiles to:**

```asm
JNCR off8
```

**Operation:** Adds the signed offset to `PC` when `C` is clear; otherwise continues at the
following instruction.  
**Registers read:** `F`, `PC`.  
**Registers written:** `PC` when the branch is taken.  
**Scratch registers:** None.  
**Stack:** None.  
**Flags:** Reads: `C`; writes: None.

### JNCR [imm16]

**Kind:** Pseudo.  
**Operands:** `imm16` is an absolute target address converted to a relative offset.  
**Compiles to:**

```asm
JNCR rel8(imm16)
```

**Operation:** Branches to `imm16` when `C` is clear. The target must fit a signed 8-bit offset
from the following instruction.  
**Registers read:** `F`, `PC`.  
**Registers written:** `PC` when the branch is taken.  
**Scratch registers:** None.  
**Stack:** None.  
**Flags:** Reads: `C`; writes: None.

### JNCR [label]

**Kind:** Pseudo.  
**Operands:** `label` is converted to a relative offset.  
**Compiles to:**

```asm
JNCR rel8(label)
```

**Operation:** Branches to `label` when `C` is clear. The target must fit a signed 8-bit offset
from the following instruction.  
**Registers read:** `F`, `PC`.  
**Registers written:** `PC` when the branch is taken.  
**Scratch registers:** None.  
**Stack:** None.  
**Flags:** Reads: `C`; writes: None.

## JNZR

### JNZR off8

**Kind:** [Core](../hard/isa.md#jnzr).  
**Operands:** `off8` is the encoded signed offset from the following instruction.  
**Compiles to:**

```asm
JNZR off8
```

**Operation:** Adds the signed offset to `PC` when `Z` is clear; otherwise continues at the
following instruction.  
**Registers read:** `F`, `PC`.  
**Registers written:** `PC` when the branch is taken.  
**Scratch registers:** None.  
**Stack:** None.  
**Flags:** Reads: `Z`; writes: None.

### JNZR [imm16]

**Kind:** Pseudo.  
**Operands:** `imm16` is an absolute target address converted to a relative offset.  
**Compiles to:**

```asm
JNZR rel8(imm16)
```

**Operation:** Branches to `imm16` when `Z` is clear. The target must fit a signed 8-bit offset
from the following instruction.  
**Registers read:** `F`, `PC`.  
**Registers written:** `PC` when the branch is taken.  
**Scratch registers:** None.  
**Stack:** None.  
**Flags:** Reads: `Z`; writes: None.

### JNZR [label]

**Kind:** Pseudo.  
**Operands:** `label` is converted to a relative offset.  
**Compiles to:**

```asm
JNZR rel8(label)
```

**Operation:** Branches to `label` when `Z` is clear. The target must fit a signed 8-bit offset
from the following instruction.  
**Registers read:** `F`, `PC`.  
**Registers written:** `PC` when the branch is taken.  
**Scratch registers:** None.  
**Stack:** None.  
**Flags:** Reads: `Z`; writes: None.

## JR

### JR off8

**Kind:** [Core](../hard/isa.md#jr).  
**Operands:** `off8` is the encoded signed offset from the following instruction.  
**Compiles to:**

```asm
JR off8
```

**Operation:** Unconditionally adds the signed offset to `PC`.  
**Registers read:** `PC`.  
**Registers written:** `PC`.  
**Scratch registers:** None.  
**Stack:** None.  
**Flags:** Reads: None; writes: None.

### JR [imm16]

**Kind:** Pseudo.  
**Operands:** `imm16` is an absolute target address converted to a relative offset.  
**Compiles to:**

```asm
JR rel8(imm16)
```

**Operation:** Branches to `imm16`. The target must fit a signed 8-bit offset from the following
instruction.  
**Registers read:** `PC`.  
**Registers written:** `PC`.  
**Scratch registers:** None.  
**Stack:** None.  
**Flags:** Reads: None; writes: None.

### JR [label]

**Kind:** Pseudo.  
**Operands:** `label` is converted to a relative offset.  
**Compiles to:**

```asm
JR rel8(label)
```

**Operation:** Branches to `label`. The target must fit a signed 8-bit offset from the following
instruction.  
**Registers read:** `PC`.  
**Registers written:** `PC`.  
**Scratch registers:** None.  
**Stack:** None.  
**Flags:** Reads: None; writes: None.

## JZR

### JZR off8

**Kind:** [Core](../hard/isa.md#jzr).  
**Operands:** `off8` is the encoded signed offset from the following instruction.  
**Compiles to:**

```asm
JZR off8
```

**Operation:** Adds the signed offset to `PC` when `Z` is set; otherwise continues at the following
instruction.  
**Registers read:** `F`, `PC`.  
**Registers written:** `PC` when the branch is taken.  
**Scratch registers:** None.  
**Stack:** None.  
**Flags:** Reads: `Z`; writes: None.

### JZR [imm16]

**Kind:** Pseudo.  
**Operands:** `imm16` is an absolute target address converted to a relative offset.  
**Compiles to:**

```asm
JZR rel8(imm16)
```

**Operation:** Branches to `imm16` when `Z` is set. The target must fit a signed 8-bit offset from
the following instruction.  
**Registers read:** `F`, `PC`.  
**Registers written:** `PC` when the branch is taken.  
**Scratch registers:** None.  
**Stack:** None.  
**Flags:** Reads: `Z`; writes: None.

### JZR [label]

**Kind:** Pseudo.  
**Operands:** `label` is converted to a relative offset.  
**Compiles to:**

```asm
JZR rel8(label)
```

**Operation:** Branches to `label` when `Z` is set. The target must fit a signed 8-bit offset from
the following instruction.  
**Registers read:** `F`, `PC`.  
**Registers written:** `PC` when the branch is taken.  
**Scratch registers:** None.  
**Stack:** None.  
**Flags:** Reads: `Z`; writes: None.

## LD

### LD rD, [rH:rL]

**Kind:** [Core](../hard/isa.md#ld).  
**Operands:** `rD` is the destination; `rH:rL` contains the source address.  
**Compiles to:**

```asm
LD rD, [rH:rL]
```

**Operation:** Reads one byte from memory at `rH:rL` into `rD`.  
**Registers read:** `rH`, `rL`.  
**Registers written:** `rD`.  
**Scratch registers:** None.  
**Stack:** None.  
**Flags:** Reads: None; writes: None.

### LD rD, [imm16]

**Kind:** Pseudo.  
**Operands:** `rD` is the destination; `imm16` is the source address.  
**Compiles to:**

```asm
LDI IH, hi(imm16)
LDI IL, lo(imm16)
LD rD, [IH:IL]
```

**Operation:** Loads the address into `IH:IL` and reads one byte from it into `rD`.  
**Registers read:** `IH`, `IL`.  
**Registers written:** `IH`, `IL`, `rD`.  
**Scratch registers:** `IH`, `IL` are clobbered.  
**Stack:** None.  
**Flags:** Reads: None; writes: None.

### LD rD, [label]

**Kind:** Pseudo.  
**Operands:** `rD` is the destination; `label` identifies the source address.  
**Compiles to:**

```asm
LDI IH, hi(label)
LDI IL, lo(label)
LD rD, [IH:IL]
```

**Operation:** Loads the address of `label` into `IH:IL` and reads one byte from it into `rD`.  
**Registers read:** `IH`, `IL`.  
**Registers written:** `IH`, `IL`, `rD`.  
**Scratch registers:** `IH`, `IL` are clobbered.  
**Stack:** None.  
**Flags:** Reads: None; writes: None.

### LD rD, [rH:rL - imm8]

**Kind:** Pseudo.  
**Operands:** `rD` is the destination; `rH:rL` is the base address; `imm8` is subtracted from it.  
**Compiles to:**

```asm
LDI A, imm8
SUB rL, A
JNCR rel8(__mb8_ld_no_borrow_{id})
PUSH A
LDI A, 0x01
SUB rH, A
POP A
__mb8_ld_no_borrow_{id}:
LD rD, [rH:rL]
```

**Operation:** Subtracts `imm8` from `rH:rL`, leaves the adjusted address in the pair, and reads
one byte from that address into `rD`.  
**Registers read:** `rH`, `rL`, `A`, `F`, `PC`, `SPH`, `SPL`.  
**Registers written:** `rH` conditionally, `rL`, `rD`, `A`, `F`, `PC` conditionally, `SPH`, `SPL`.  
**Scratch registers:** `A` is clobbered with `imm8`.  
**Stack:** The borrow path uses one temporary byte; maximum depth is 1 byte and net stack-pointer
change is 0.  
**Flags:** Reads: `C` for the generated branch; writes: `Z`, `N`, and `C` from subtracting
`imm8` from `rL`, or from decrementing `rH` when a borrow occurs.

## LDI

### LDI rD, imm8

**Kind:** [Core](../hard/isa.md#ldi).  
**Operands:** `rD` is the destination; `imm8` is the byte to load.  
**Compiles to:**

```asm
LDI rD, imm8
```

**Operation:** Stores `imm8` in `rD`.  
**Registers read:** None.  
**Registers written:** `rD`.  
**Scratch registers:** None.  
**Stack:** None.  
**Flags:** Reads: None; writes: None.

### LDI rH:rL, imm16

**Kind:** Pseudo.  
**Operands:** `rH:rL` is the destination pair; `imm16` is the 16-bit value.  
**Compiles to:**

```asm
LDI rH, hi(imm16)
LDI rL, lo(imm16)
```

**Operation:** Stores the high byte of `imm16` in `rH` and the low byte in `rL`.  
**Registers read:** None.  
**Registers written:** `rH`, `rL`.  
**Scratch registers:** None.  
**Stack:** None.  
**Flags:** Reads: None; writes: None.

### LDI rH:rL, label

**Kind:** Pseudo.  
**Operands:** `rH:rL` is the destination pair; `label` supplies its 16-bit address.  
**Compiles to:**

```asm
LDI rH, hi(label)
LDI rL, lo(label)
```

**Operation:** Stores the high and low bytes of the address of `label` in `rH:rL`.  
**Registers read:** None.  
**Registers written:** `rH`, `rL`.  
**Scratch registers:** None.  
**Stack:** None.  
**Flags:** Reads: None; writes: None.

## MEMCPY

### MEMCPY [dstH:dstL], [srcH:srcL], len

**Kind:** Pseudo.  
**Operands:** `dstH:dstL` is the destination pointer, `srcH:srcL` is the source pointer, and `len`
is a register containing the last zero-based byte index.  
**Compiles to:**

```asm
PUSH A
LDI A, 0x00
__mb8_memcpy_loop_{id}:
PUSH A
LD A, [srcH:srcL]
ST [dstH:dstL], A
POP A
CMP A, len
JZR rel8(__mb8_memcpy_end_{id})
PUSH R7
LDI R7, 0x01
ADD A, R7
POP R7

PUSH A
LDI A, 0xFF
CMP srcL, A
POP A
JZR rel8(__mb8_memcpy_src_{id}_hi)
PUSH A
LDI A, 0x01
ADD srcL, A
POP A
JR rel8(__mb8_memcpy_src_{id}_end)
__mb8_memcpy_src_{id}_hi:
LDI srcL, 0x00
PUSH A
LDI A, 0x01
ADD srcH, A
POP A
__mb8_memcpy_src_{id}_end:
NOP

PUSH A
LDI A, 0xFF
CMP dstL, A
POP A
JZR rel8(__mb8_memcpy_dst_{id}_hi)
PUSH A
LDI A, 0x01
ADD dstL, A
POP A
JR rel8(__mb8_memcpy_dst_{id}_end)
__mb8_memcpy_dst_{id}_hi:
LDI dstL, 0x00
PUSH A
LDI A, 0x01
ADD dstH, A
POP A
__mb8_memcpy_dst_{id}_end:
NOP

JR rel8(__mb8_memcpy_loop_{id})
__mb8_memcpy_end_{id}:
POP A
```

**Operation:** Copies `len + 1` bytes. The source and destination pointers advance after every
byte except the last; `A` and `R7` are restored.  
**Registers read:** `dstH`, `dstL`, `srcH`, `srcL`, `len`, `A`, `R7`, `F`, `PC`, `SPH`, `SPL`.  
**Registers written:** `dstH`, `dstL`, `srcH`, `srcL`, `A`, `R7`, `F`, `PC`, `SPH`, `SPL`.  
**Scratch registers:** `A` and `R7` are temporarily modified and restored.  
**Stack:** Keeps the original `A` on the stack for the loop and uses one additional temporary byte;
maximum depth is 2 bytes and net stack-pointer change is 0.  
**Flags:** Reads: `Z` and `C` in generated branches; writes: final `Z = 1`, `N = 0`, and `C = 0`
from the terminating equality comparison of the byte index with `len`.

## MOV

### MOV rD, rS

**Kind:** [Core](../hard/isa.md#mov).  
**Operands:** `rD` is the destination; `rS` is the source.  
**Compiles to:**

```asm
MOV rD, rS
```

**Operation:** Copies the byte in `rS` to `rD`.  
**Registers read:** `rS`.  
**Registers written:** `rD`.  
**Scratch registers:** None.  
**Stack:** None.  
**Flags:** Reads: None; writes: None.

## MUL

### MUL rD, rA, rB

**Kind:** Pseudo.  
**Operands:** `rD` receives the product; `rA` is the repeated addend; `rB` is the iteration count.  
**Compiles to:**

```asm
LDI rD, 0x00
PUSH rB
__mb8_mul_iter_{id}:
ADD rD, rA
PUSH A
LDI A, 0x01
SUB rB, A
POP A
PUSH A
LDI A, 0x00
CMP rB, A
POP A
JNZR rel8(__mb8_mul_iter_{id})
POP rB
```

**Operation:** Repeatedly adds `rA` to `rD` while decrementing `rB` to zero. The 8-bit result wraps;
the original values of `rB` and `A` are restored. An initial `rB` of zero performs 256 iterations.  
**Registers read:** `rA`, `rB`, `A`, `F`, `PC`, `SPH`, `SPL`.  
**Registers written:** `rD`, `rB`, `A`, `F`, `PC`, `SPH`, `SPL`.  
**Scratch registers:** `A` and `rB` are temporarily modified and restored.  
**Stack:** Keeps the original `rB` on the stack and uses one additional temporary byte; maximum
depth is 2 bytes and net stack-pointer change is 0.  
**Flags:** Reads: `Z` for the generated loop branch; writes: final `Z = 1`, `N = 0`, and `C = 0`
from comparing the decremented `rB` with zero.

## NOP

### NOP

**Kind:** [Core](../hard/isa.md#nop).  
**Operands:** None.  
**Compiles to:**

```asm
NOP
```

**Operation:** Advances to the following instruction without changing processor state.  
**Registers read:** None.  
**Registers written:** None.  
**Scratch registers:** None.  
**Stack:** None.  
**Flags:** Reads: None; writes: None.

## NOT

### NOT rD

**Kind:** Pseudo.  
**Operands:** `rD` is the register to invert.  
**Compiles to:**

```asm
PUSH A
LDI A, 0xFF
XOR rD, A
POP A
```

**Operation:** Inverts every bit of `rD` while preserving `A`.  
**Registers read:** `rD`, `A`, `SPH`, `SPL`.  
**Registers written:** `rD`, `A`, `F`, `SPH`, `SPL`.  
**Scratch registers:** `A` is temporarily modified and restored.  
**Stack:** Uses one temporary byte; maximum depth is 1 byte and net stack-pointer change is 0.  
**Flags:** Reads: None; writes: `Z` when the inverted result is zero, `N` from result bit 7, and
clears `C`.

## OR

### OR rD, rS

**Kind:** [Core](../hard/isa.md#or).  
**Operands:** `rD` is the destination and left operand; `rS` is the right operand.  
**Compiles to:**

```asm
OR rD, rS
```

**Operation:** Stores the bitwise OR of `rD` and `rS` in `rD`.  
**Registers read:** `rD`, `rS`.  
**Registers written:** `rD`, `F`.  
**Scratch registers:** None.  
**Stack:** None.  
**Flags:** Reads: None; writes: `Z` when the result is zero, `N` from result bit 7, and clears `C`.

## POP

### POP rD

**Kind:** [Core](../hard/isa.md#pop).  
**Operands:** `rD` is the destination register.  
**Compiles to:**

```asm
POP rD
```

**Operation:** Increments the stack pointer and loads the byte at the new address into `rD`; halts
the VM on stack underflow.  
**Registers read:** `SPH`, `SPL`.  
**Registers written:** `rD`, `SPH`, `SPL`.  
**Scratch registers:** None.  
**Stack:** Pops one byte; net stack-pointer change is +1 byte.  
**Flags:** Reads: None; writes: None.

## PUSH

### PUSH rS

**Kind:** [Core](../hard/isa.md#push).  
**Operands:** `rS` is the source register.  
**Compiles to:**

```asm
PUSH rS
```

**Operation:** Writes `rS` at the current stack pointer and decrements it; halts the VM on stack
overflow.  
**Registers read:** `rS`, `SPH`, `SPL`.  
**Registers written:** `SPH`, `SPL`.  
**Scratch registers:** None.  
**Stack:** Pushes one byte; net stack-pointer change is -1 byte.  
**Flags:** Reads: None; writes: None.

## RET

### RET

**Kind:** [Core](../hard/isa.md#ret).  
**Operands:** None.  
**Compiles to:**

```asm
RET
```

**Operation:** Pops a two-byte return address into `PC`; halts the VM on stack underflow.  
**Registers read:** `SPH`, `SPL`.  
**Registers written:** `PC`, `SPH`, `SPL`.  
**Scratch registers:** None.  
**Stack:** Pops two bytes; net stack-pointer change is +2 bytes.  
**Flags:** Reads: None; writes: None.

## SHL

### SHL rD, rS

**Kind:** [Core](../hard/isa.md#shl).  
**Operands:** `rD` is the value and destination; `rS` contains the shift count.  
**Compiles to:**

```asm
SHL rD, rS
```

**Operation:** Repeatedly shifts `rD` left by the count in `rS`, discarding high bits.  
**Registers read:** `rD`, `rS`.  
**Registers written:** `rD`, `F`.  
**Scratch registers:** None.  
**Stack:** None.  
**Flags:** Reads: None; writes: `Z` when the result is zero, `N` from result bit 7, and `C` from
the last high bit shifted out; `C` is clear for a zero shift count.

### SHL rD, imm8

**Kind:** Pseudo.  
**Operands:** `rD` is the value and destination; `imm8` is the shift count.  
**Compiles to:**

```asm
PUSH A
LDI A, imm8
SHL rD, A
POP A
```

**Operation:** Shifts `rD` left by `imm8`, preserving the original value of `A`.  
**Registers read:** `rD`, `A`, `SPH`, `SPL`.  
**Registers written:** `rD`, `A`, `F`, `SPH`, `SPL`.  
**Scratch registers:** `A` is temporarily modified and restored.  
**Stack:** Uses one temporary byte; maximum depth is 1 byte and net stack-pointer change is 0.  
**Flags:** Reads: None; writes: `Z` when the result is zero, `N` from result bit 7, and `C` from
the last high bit shifted out; `C` is clear for a zero shift count.

## SHR

### SHR rD, rS

**Kind:** [Core](../hard/isa.md#shr).  
**Operands:** `rD` is the value and destination; `rS` contains the shift count.  
**Compiles to:**

```asm
SHR rD, rS
```

**Operation:** Repeatedly shifts `rD` right logically by the count in `rS`, inserting zero bits.  
**Registers read:** `rD`, `rS`.  
**Registers written:** `rD`, `F`.  
**Scratch registers:** None.  
**Stack:** None.  
**Flags:** Reads: None; writes: `Z` when the result is zero, `N` from result bit 7, and `C` from
the last low bit shifted out; `C` is clear for a zero shift count.

### SHR rD, imm8

**Kind:** Pseudo.  
**Operands:** `rD` is the value and destination; `imm8` is the shift count.  
**Compiles to:**

```asm
PUSH A
LDI A, imm8
SHR rD, A
POP A
```

**Operation:** Shifts `rD` right logically by `imm8`, preserving the original value of `A`.  
**Registers read:** `rD`, `A`, `SPH`, `SPL`.  
**Registers written:** `rD`, `A`, `F`, `SPH`, `SPL`.  
**Scratch registers:** `A` is temporarily modified and restored.  
**Stack:** Uses one temporary byte; maximum depth is 1 byte and net stack-pointer change is 0.  
**Flags:** Reads: None; writes: `Z` when the result is zero, `N` from result bit 7, and `C` from
the last low bit shifted out; `C` is clear for a zero shift count.

## ST

### ST [rH:rL], rS

**Kind:** [Core](../hard/isa.md#st).  
**Operands:** `rH:rL` contains the destination address; `rS` contains the byte to store.  
**Compiles to:**

```asm
ST [rH:rL], rS
```

**Operation:** Writes `rS` to memory at the address in `rH:rL`.  
**Registers read:** `rH`, `rL`, `rS`.  
**Registers written:** None.  
**Scratch registers:** None.  
**Stack:** None.  
**Flags:** Reads: None; writes: None.

### ST [imm16], rS

**Kind:** Pseudo.  
**Operands:** `imm16` is the destination address; `rS` contains the byte to store.  
**Compiles to:**

```asm
LDI IH, hi(imm16)
LDI IL, lo(imm16)
ST [IH:IL], rS
```

**Operation:** Loads `imm16` into `IH:IL` and writes `rS` to that address.  
**Registers read:** `IH`, `IL`, `rS`.  
**Registers written:** `IH`, `IL`.  
**Scratch registers:** `IH`, `IL` are clobbered.  
**Stack:** None.  
**Flags:** Reads: None; writes: None.

### ST [label], rS

**Kind:** Pseudo.  
**Operands:** `label` identifies the destination address; `rS` contains the byte to store.  
**Compiles to:**

```asm
LDI IH, hi(label)
LDI IL, lo(label)
ST [IH:IL], rS
```

**Operation:** Loads the address of `label` into `IH:IL` and writes `rS` to that address.  
**Registers read:** `IH`, `IL`, `rS`.  
**Registers written:** `IH`, `IL`.  
**Scratch registers:** `IH`, `IL` are clobbered.  
**Stack:** None.  
**Flags:** Reads: None; writes: None.

## STRCMP

### STRCMP result, temp, srcH, srcL, dstH, dstL

**Kind:** Pseudo.  
**Operands:** `srcH:srcL` and `dstH:dstL` point to zero-terminated strings; `result` receives the
comparison result; `temp` receives bytes from the second string.  
**Compiles to:**

```asm
__mb8_strcmp_loop_{id}:
LD result, [srcH:srcL]
LD temp, [dstH:dstL]
CMP result, temp
JNZR rel8(__mb8_strcmp_error_{id})
PUSH A
LDI A, 0x00
CMP temp, A
POP A
JZR rel8(__mb8_strcmp_success_{id})

PUSH A
LDI A, 0xFF
CMP srcL, A
POP A
JZR rel8(__mb8_strcmp_src_{id}_hi)
PUSH A
LDI A, 0x01
ADD srcL, A
POP A
JR rel8(__mb8_strcmp_src_{id}_end)
__mb8_strcmp_src_{id}_hi:
LDI srcL, 0x00
PUSH A
LDI A, 0x01
ADD srcH, A
POP A
__mb8_strcmp_src_{id}_end:
NOP

PUSH A
LDI A, 0xFF
CMP dstL, A
POP A
JZR rel8(__mb8_strcmp_dst_{id}_hi)
PUSH A
LDI A, 0x01
ADD dstL, A
POP A
JR rel8(__mb8_strcmp_dst_{id}_end)
__mb8_strcmp_dst_{id}_hi:
LDI dstL, 0x00
PUSH A
LDI A, 0x01
ADD dstH, A
POP A
__mb8_strcmp_dst_{id}_end:
NOP

LDI IH, hi(__mb8_strcmp_loop_{id})
LDI IL, lo(__mb8_strcmp_loop_{id})
JMP [IH:IL]
__mb8_strcmp_error_{id}:
LDI result, 0x01
JR rel8(__mb8_strcmp_end_{id})
__mb8_strcmp_success_{id}:
LDI result, 0x00
__mb8_strcmp_end_{id}:
```

**Operation:** Compares the strings byte by byte. Writes `0` to `result` when both reach the same
zero terminator and `1` on the first mismatch. Address pairs advance after equal nonzero bytes;
`temp` retains the last byte read from the second string.  
**Registers read:** `srcH`, `srcL`, `dstH`, `dstL`, `result`, `temp`, `A`, `IH`, `IL`, `F`, `PC`,
`SPH`, `SPL`.  
**Registers written:** `srcH`, `srcL`, `dstH`, `dstL`, `result`, `temp`, `A`, `IH`, `IL`, `F`,
`PC`, `SPH`, `SPL`.  
**Scratch registers:** `A` is temporarily modified and restored; `IH` and `IL` are clobbered when
the loop repeats.  
**Stack:** Uses one temporary byte at a time; maximum depth is 1 byte and net stack-pointer change
is 0.  
**Flags:** Reads: `Z` and `C` in generated branches; writes: on mismatch, retains `Z = 0` with `N`
and `C` from `result - temp`; on equality, ends with `Z = 1`, `N = 0`, and `C = 0` from comparing
the zero terminator with zero.

## SUB

### SUB rD, rS

**Kind:** [Core](../hard/isa.md#sub).  
**Operands:** `rD` is the destination and left operand; `rS` is the right operand.  
**Compiles to:**

```asm
SUB rD, rS
```

**Operation:** Subtracts `rS` from `rD`, stores the wrapped 8-bit result in `rD`, and updates
arithmetic flags.  
**Registers read:** `rD`, `rS`.  
**Registers written:** `rD`, `F`.  
**Scratch registers:** None.  
**Stack:** None.  
**Flags:** Reads: None; writes: `Z` when the result is zero, `N` from result bit 7, and `C` on
unsigned underflow.

## SWAP

### SWAP rA, rB

**Kind:** Pseudo.  
**Operands:** `rA` and `rB` are the registers to exchange.  
**Compiles to:**

```asm
PUSH rA
MOV rA, rB
POP rB
```

**Operation:** Exchanges the values in `rA` and `rB`.  
**Registers read:** `rA`, `rB`, `SPH`, `SPL`.  
**Registers written:** `rA`, `rB`, `SPH`, `SPL`.  
**Scratch registers:** None.  
**Stack:** Uses one temporary byte; maximum depth is 1 byte and net stack-pointer change is 0.  
**Flags:** Reads: None; writes: None.

## SYS

### SYS

**Kind:** [Core](../hard/isa.md#sys).  
**Operands:** None.  
**Compiles to:**

```asm
SYS
```

**Operation:** Executes the reserved system opcode. Its VM operation is currently unimplemented.  
**Registers read:** None.  
**Registers written:** None.  
**Scratch registers:** None.  
**Stack:** None.  
**Flags:** Reads: None; writes: None.

## XOR

### XOR rD, rS

**Kind:** [Core](../hard/isa.md#xor).  
**Operands:** `rD` is the destination and left operand; `rS` is the right operand.  
**Compiles to:**

```asm
XOR rD, rS
```

**Operation:** Stores the bitwise XOR of `rD` and `rS` in `rD`.  
**Registers read:** `rD`, `rS`.  
**Registers written:** `rD`, `F`.  
**Scratch registers:** None.  
**Stack:** None.  
**Flags:** Reads: None; writes: `Z` when the result is zero, `N` from result bit 7, and clears `C`.

## ZERO

### ZERO rD

**Kind:** Pseudo.  
**Operands:** `rD` is the register to clear.  
**Compiles to:**

```asm
LDI rD, 0x00
```

**Operation:** Stores zero in `rD`.  
**Registers read:** None.  
**Registers written:** `rD`.  
**Scratch registers:** None.  
**Stack:** None.  
**Flags:** Reads: None; writes: None.
