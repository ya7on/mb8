# Memory model

The MB8 VM exposes a single 64 KiB address space per context. The bus (`crates/mb8/src/dev/bus.rs`) routes requests from the CPU to RAM, ROM, and MMIO regions according to the high bits of the address.

## Layout

| Range | Size | Description |
| --- | --- | --- |
| `0x0000` – `0xBFFF` | 48 KiB | Read/write RAM. General data, stack, and temporary buffers live here. |
| `0xC000` – `0xDFFF` | 8 KiB | Reserved MMIO window (devices WIP). Reads/writes are not routed yet. |
| `0xE000` – `0xEFFF` | 4 KiB | ROM. Holds 16-bit instructions loaded from cartridges. Read-only at runtime. |
| `0xF000` – `0xFFFF` | 4 KiB | Reserved MMIO window (future peripherals). |

## RAM details

- `RAM_SIZE` is `0xC000` bytes. Addressing is byte-based even though instructions are 16 bits.
- The stack grows downward. `STACK_TOP = 0xBFFF` and `STACK_BOTTOM = 0xBF00`. Calls push the current `PC` byte-by-byte starting from `STACK_TOP`, and stack overflow halts the VM.
- The rest of RAM (`0x0000` – `0xBEFF`) is free-form working memory for programs.

## ROM

- Each VM instance has its own ROM buffer with `ROM_SIZE = 0x1000` bytes.
- The `PC` reads 16-bit opcodes from ROM. Self-modifying code is impossible because ROM writes are ignored by the CPU.
- Absolute control-flow instructions (`JMP`, `CALL`) combine two registers into a 16-bit ROM address.

## MMIO windows

The two MMIO ranges (`0xC000`–`0xDFFF` and `0xF000`–`0xFFFF`) are mapped by the bus but currently stubbed out. They reserve space for future devices such as timers, input, or IPC mailboxes. Reads/writes to these pages should be avoided until the corresponding devices are implemented.
