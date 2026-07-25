# Instruction set

MB8 instructions are 16 bits wide. This page describes the opcodes decoded and executed by the
processor. Textual forms accepted by the assembler are documented separately in the
[assembler instruction reference](../asm/instructions.md).

## Encoding notation

- `D` and `S` identify destination and source registers.
- `H` and `L` identify registers containing the high and low bytes of an address.
- `I` is an unsigned immediate bit.
- `O` is a signed relative-offset bit.
- `X` is ignored by the decoder.
- Register fields use the four-bit values from the [register set](registers.md).
- Unless stated otherwise, an instruction leaves the flags unchanged.

## Control instructions

### NOP

**Encoding:** `0000 0000 XXXX XXXX`  
**Hex pattern:** `0x00XX`  
**Operation:** none.  
**Behavior:** Advances execution without changing processor state.

### HALT

**Encoding:** `0000 0001 XXXX XXXX`  
**Hex pattern:** `0x01XX`  
**Operation:** `halted ← true`.  
**Behavior:** Stops the VM. The low byte is ignored by the current processor implementation.

### SYS

**Encoding:** `0000 0010 XXXX XXXX`  
**Hex pattern:** `0x02XX`  
**Operation:** reserved system operation.  
**Behavior:** The opcode is decoded by the processor, but its VM operation is currently
unimplemented.

## Register-register instructions

### MOV

**Encoding:** `0001 0000 DDDD SSSS`  
**Hex pattern:** `0x10DS`  
**Fields:** `D` is the destination register; `S` is the source register.

```text
R[D] ← R[S]
```

Copies one register to another.

### ADD

**Encoding:** `0001 0001 DDDD SSSS`  
**Hex pattern:** `0x11DS`  
**Fields:** `D` is the destination register; `S` is the source register.

```text
R[D] ← R[D] + R[S]
```

The 8-bit result wraps on overflow.

**Flags:** replaces `Z`, `N`, and `C`; `C` indicates unsigned overflow.

### SUB

**Encoding:** `0001 0010 DDDD SSSS`  
**Hex pattern:** `0x12DS`  
**Fields:** `D` is the destination register; `S` is the source register.

```text
R[D] ← R[D] - R[S]
```

The 8-bit result wraps on underflow.

**Flags:** replaces `Z`, `N`, and `C`; `C` indicates unsigned underflow.

### AND

**Encoding:** `0001 0011 DDDD SSSS`  
**Hex pattern:** `0x13DS`  
**Fields:** `D` is the destination register; `S` is the source register.

```text
R[D] ← R[D] AND R[S]
```

**Flags:** replaces `Z` and `N`; clears `C`.

### OR

**Encoding:** `0001 0100 DDDD SSSS`  
**Hex pattern:** `0x14DS`  
**Fields:** `D` is the destination register; `S` is the source register.

```text
R[D] ← R[D] OR R[S]
```

**Flags:** replaces `Z` and `N`; clears `C`.

### XOR

**Encoding:** `0001 0101 DDDD SSSS`  
**Hex pattern:** `0x15DS`  
**Fields:** `D` is the destination register; `S` is the source register.

```text
R[D] ← R[D] XOR R[S]
```

**Flags:** replaces `Z` and `N`; clears `C`.

### SHR

**Encoding:** `0001 0110 DDDD SSSS`  
**Hex pattern:** `0x16DS`  
**Fields:** `D` is the value register; `S` contains the shift count.

```text
R[D] ← R[D] >> R[S]
```

Performs repeated logical right shifts.

**Flags:** replaces `Z`, `N`, and `C`; `C` contains the last bit shifted out, or is clear when the
shift count is zero.

### SHL

**Encoding:** `0001 0111 DDDD SSSS`  
**Hex pattern:** `0x17DS`  
**Fields:** `D` is the value register; `S` contains the shift count.

```text
R[D] ← R[D] << R[S]
```

Performs repeated 8-bit left shifts.

**Flags:** replaces `Z`, `N`, and `C`; `C` contains the last bit shifted out, or is clear when the
shift count is zero.

### CMP

**Encoding:** `0001 1000 DDDD SSSS`  
**Hex pattern:** `0x18DS`  
**Fields:** `D` and `S` identify the registers to compare.

```text
result ← R[D] - R[S]
```

Computes a subtraction for its flags without modifying either operand.

**Flags:** replaces `Z`, `N`, and `C`; `C` indicates unsigned underflow.

## Immediate instructions

### LDI

**Encoding:** `0010 DDDD IIII IIII`  
**Hex pattern:** `0x2DII`  
**Fields:** `D` is the destination register; `I` is an unsigned 8-bit value.

```text
R[D] ← I
```

Loads an immediate byte into a register.

## Jump instructions

Relative offsets are signed 8-bit values. The VM applies them to the program counter value for the
next instruction.

### JMP

**Encoding:** `0011 0000 HHHH LLLL`  
**Hex pattern:** `0x30HL`  
**Fields:** `H` and `L` identify registers containing the high and low address bytes.

```text
PC ← (R[H] << 8) OR R[L]
```

Performs an absolute jump.

### JR

**Encoding:** `0011 0001 OOOO OOOO`  
**Hex pattern:** `0x31OO`

```text
PC ← PC + sign_extend(O)
```

Performs an unconditional relative jump with 16-bit wrapping.

### JZR

**Encoding:** `0011 0010 OOOO OOOO`  
**Hex pattern:** `0x32OO`

```text
if Z = 1: PC ← PC + sign_extend(O)
```

Performs a relative jump when the zero flag is set.

### JNZR

**Encoding:** `0011 0011 OOOO OOOO`  
**Hex pattern:** `0x33OO`

```text
if Z = 0: PC ← PC + sign_extend(O)
```

Performs a relative jump when the zero flag is clear.

### JCR

**Encoding:** `0011 0100 OOOO OOOO`  
**Hex pattern:** `0x34OO`

```text
if C = 1: PC ← PC + sign_extend(O)
```

Performs a relative jump when the carry flag is set.

### JNCR

**Encoding:** `0011 0101 OOOO OOOO`  
**Hex pattern:** `0x35OO`

```text
if C = 0: PC ← PC + sign_extend(O)
```

Performs a relative jump when the carry flag is clear.

## Stack instructions

The stack pointer is the 16-bit value in `SPH:SPL`. Stack operations halt the VM when they cross
the implemented stack bounds.

### CALL

**Encoding:** `0100 0000 HHHH LLLL`  
**Hex pattern:** `0x40HL`  
**Fields:** `H` and `L` identify registers containing the high and low target-address bytes.

```text
push16(PC)
PC ← (R[H] << 8) OR R[L]
```

Pushes the return address and transfers control to an absolute address.

### RET

**Encoding:** `0100 0001 XXXX XXXX`  
**Hex pattern:** `0x41XX`

```text
PC ← pop16()
```

Restores a return address from the stack.

### PUSH

**Encoding:** `0100 0010 SSSS XXXX`  
**Hex pattern:** `0x42SX`  
**Fields:** `S` is the source register.

```text
MEM[SP] ← R[S]
SP ← SP - 1
```

Pushes one byte onto the descending stack.

### POP

**Encoding:** `0100 0011 DDDD XXXX`  
**Hex pattern:** `0x43DX`  
**Fields:** `D` is the destination register.

```text
SP ← SP + 1
R[D] ← MEM[SP]
```

Pops one byte from the descending stack.

## Memory instructions

### LD

**Encoding:** `0101 DDDD HHHH LLLL`  
**Hex pattern:** `0x5DHL`  
**Fields:** `D` is the destination; `H` and `L` identify the address registers.

```text
R[D] ← MEM[(R[H] << 8) OR R[L]]
```

Reads one byte through the memory bus.

### ST

**Encoding:** `0110 SSSS HHHH LLLL`  
**Hex pattern:** `0x6SHL`  
**Fields:** `S` is the source; `H` and `L` identify the address registers.

```text
MEM[(R[H] << 8) OR R[L]] ← R[S]
```

Writes one byte through the memory bus.
