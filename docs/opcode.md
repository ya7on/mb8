# Instruction format

Every opcode is 16 bits wide (`0xABCD`):
- `A` — instruction group.
- `B` — sub-opcode or register nibble.
- `C` — usually a register or the upper 4 bits of an address.
- `D` — usually a register or the lower 4 bits of an address.

Example, `ADD R0, R1` encodes as `0x1101`:
```
0001 0001 0000 0001
```

Jump/load/store instructions treat `XXX` in `0xYXXX` as a 12-bit address, covering the full 4 KiB memory bank.
