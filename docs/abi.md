# Data model

## Types

- `int` - 16 bits
- `char` - 8 bits
- `pointer` - 16 bits

## Registers

- **Callee saved**: `R0 | A`, `R2`, `R4`, `R5`, `R8`, `R9 | IH`, `R10 | IL`.
- **Caller saved**: `R1`, `R3`, `R6`, `R7`, `R11 | FPH`, `R12 | FPL`.
- **System**: `R13 | SPH`, `R14 | SPL`, `R15 | F`.

Calling rules:
- Function arguments: first three in `R1`, `R2`, `R3`; the rest on the stack.
- Return value: always in `R0` (`A`).
- Frame pointer: 16-bit value in `FPH:FPL`.
- Stack pointer: 16-bit value in `SPH:SPL`; stack starts at `0xBFFF` and grows downward.
- Index register: `IH:IL` is a free 16-bit index pair for addressing.
