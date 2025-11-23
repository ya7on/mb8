# Standard Library

Helper macros from `asm/std.asm`. Include them after `cpu.asm`/`ext.asm` to get simple data routines.

## MEMCPY
- **Syntax**: `MEMCPY i len srchi srclo dsthi dstlo`
- **Inputs**: `i` loop counter (start at 0), `len` stop value, `srchi:srclo` source pointer, `dsthi:dstlo` destination pointer.
- **Behavior**: Copies bytes from source to destination until `i == len`. Both pointers are incremented with `INC16`. Restores `i` each iteration via push/pop.
- **Scratch**: uses stack to save `i`; flags from `CMP`, `INC`, `INC16`.

## STRCMP
- **Syntax**: `STRCMP i j srchi srclo dsthi dstlo`
- **Inputs**: `srchi:srclo` first string pointer, `dsthi:dstlo` second string pointer.
- **Outputs**: `i` receives `0` when strings match, `1` otherwise.
- **Behavior**: Walks both zero-terminated strings byte-by-byte. Returns early on mismatch, or when a `0x00` terminator is reached on both sides.
- **Scratch**: uses `i`, `j`, flags from `CMP`, `CMPI`, `JZR/JNZR`, and increments addresses with `INC16`.
