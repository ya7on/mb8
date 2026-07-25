# Assembler syntax

MB8 assembly is compiled by the Rust `asm` crate in this workspace.

## Writing a program for the VM

User images start at `0x1000`, while kernel and kernel-test images start at `0xE000`. Declare that
base once at the top of each root source:

```asm
.origin 0x1000

start:
    LDI R1, 0x03
    HALT

.addr 0x2000
```

The final `.addr` pads the image to its 4 KiB boundary.

## Instructions and operands

Instructions and registers are conventionally uppercase. Separate operands with commas and write
register pairs with a colon:

```asm
LDI R2:R3, MESSAGE
LD R1, [R2:R3]
ST [0xF001], R1
CALL [0xE500]
```

The assembler provides the ISA and pseudo-instructions directly. Pseudo-instructions include
`INC`, `DEC`, `INC16`, `MUL`, `MEMCPY`, `STRCMP`, immediate jumps, and absolute calls, loads,
stores, and jumps; no rule-file include is needed.

## Directives

- `.origin <address>` sets the image base and may appear once in the root program.
- `.addr <address>` pads with zero bytes up to an absolute address.
- `.data <byte>, ...` emits bytes.
- `.ascii "text"` emits a string; `\n`, `\0`, `\\`, and `\"` escapes are supported.
- `.include "path.asm"` inserts another source relative to the containing file.
- `.const @NAME, <value>` defines a constant; refer to it as `@NAME`.

Labels beginning with `_` are local to the preceding non-local label in the same source:

```asm
.const @SYS_WRITE, 0x02

write:
    LDI R1, @SYS_WRITE
_loop:
    JR [_loop]
```

Numeric literals use hexadecimal notation. Character immediates should be written as their byte
value, such as `0x0A` for newline.

## Building and running

Assemble one source directly:

```sh
cargo run --quiet -p asm -- user/sh.asm -o user/sh.bin
```

Use `make`, `make kernel`, `make user` for the repository images, and `make run`
to launch the VM.
