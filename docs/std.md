# Standard Library

Helper macros from `asm/std.asm`. Include them after `cpu.asm`/`ext.asm` to get simple data routines.

## MEMCPY
- **Syntax**: `MEMCPY [dsthi:dstlo] [srchi:srclo] len`
- **Inputs**: `srchi:srclo` source pointer, `dsthi:dstlo` destination pointer, `len` stop value.
- **Behavior**: Copies bytes from source to destination while an internal counter in `A` walks from `0` up to `len`. That means it copies `len + 1` bytes. Source and destination advance with `INC16`. Restores the original `A` after finishing.
- **Scratch**: uses stack to save `A`; flags from `CMP`, `INC`, `INC16`.

## STRCMP
- **Syntax**: `STRCMP i j srchi srclo dsthi dstlo`
- **Inputs**: `srchi:srclo` first string pointer, `dsthi:dstlo` second string pointer.
- **Outputs**: `i` receives `0` when strings match, `1` otherwise.
- **Behavior**: Walks both zero-terminated strings byte-by-byte. Returns early on mismatch, or when a `0x00` terminator is reached on both sides.
- **Scratch**: uses `i`, `j`, flags from `CMP`, `CMPI`, `JZR/JNZR`, and increments addresses with `INC16`.
