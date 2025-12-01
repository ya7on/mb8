# Assembler syntax

We assemble with [`customasm`](https://github.com/hlorenzi/customasm).

## Writing a program for the VM
- Always include `asm/cpu.asm` first (see `user/sh.asm`). It defines the memory banks so your ROM segment assembles with a base address of `0x1000`.
- When the program is launched, that ROM image is copied into RAM starting at `0x1000` and execution begins at your entry label.

## Includes
- Always include `asm/cpu.asm` to get the core ISA and register definitions.
- Optionally include `asm/ext.asm` to unlock pseudo-instructions like `INC`, `JMP addr`, `CMPI`, etc.

```asm
#include "../asm/cpu.asm"
#include "../asm/ext.asm" ; optional
```

## Banks and layout
- Kernel: uses `#bank rom` (4 KiB at `0xE000`) for executable code — see `kernel/main.asm`.
- User programs: include `cpu.asm` and you automatically get the RAM bank defined there; you typically do **not** write your own `#bank` directives.
- `#addr <hex>` — set the write pointer inside the active bank. Handy to place data at a specific RAM offset.

Example RAM data:
```asm
#addr 0x0100      ; place data in general RAM (RAM bank is already active)
SPRITE:
    #d8 0b1111_0000
    #d8 0b1000_1000
```

Example ROM code:
```asm
#bank rom         ; only in the kernel image
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
- Place the executable file in the `user` directory.
- Update `Makefile` with `USER_PROGRAMS += file.bin`.
- Run: `make run`
