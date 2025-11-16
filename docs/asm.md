# Assembler syntax

We assemble with [`customasm`](https://github.com/hlorenzi/customasm).

## Includes
- Always include `asm/cpu.asm` to get the core ISA and register definitions.
- Optionally include `asm/ext.asm` to unlock pseudo-instructions like `INC`, `JMPR`, `CMP`, etc.

```asm
#include "../asm/cpu.asm"
#include "../asm/ext.asm" ; optional
```

## Banks and layout
- `#bank rom` — code bank (4 KiB). **All executable instructions must live here.**
- `#bank ram` — data bank (4 KiB), laid out as stack, general RAM, and special region as described in the memory model.
- `#addr <hex>` — set the write pointer inside the active bank. Useful to prefill RAM or place sprites.

Example RAM data:
```asm
#bank ram
#addr 0x0100      ; place data in general RAM
SPRITE:
    #d8 0b1111_0000
    #d8 0b1000_1000
```

Example ROM code:
```asm
#bank rom
start:
    LDI R0 0x42
    HALT
```

## Data and literals
- `#d8` emits bytes (comma-separated or one-per-line).
- Labels mark addresses (`label:`). You can jump to labels or use them for data pointers.
- Constants use `NAME = value`.
- Literals: decimal, hex (`0xFF`), binary (`0b1010_1010`), or single-character strings (`"A"` stores ASCII).

## Comments
- Everything after `;` on a line is ignored.

## Building and running
- Build: `customasm file.asm` → produces `file.bin`.
- Run: `cargo run -- run file.bin` (add `--bot other.bin` to load a bot alongside the judge).
