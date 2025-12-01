# Memory model

The MB8 VM exposes a single 64 KiB address space. All reads and writes go through the bus (`crates/mb8/src/dev/bus.rs`), which forwards them to RAM, ROM, or an MMIO device based on the address range.

## Layout

| Range | Size | Description |
| --- | --- | --- |
| `0x0000` – `0xBFFF` | 48 KiB | RAM |
| `0xC000` – `0xDFFF` | 8 KiB | Reserved MMIO (not wired yet) |
| `0xE000` – `0xEFFF` | 4 KiB | ROM |
| `0xF000` – `0xF0FF` | 256 B | GPU registers |
| `0xF100` – `0xF1FF` | 256 B | Keyboard registers |
| `0xF200` – `0xF3FF` | 512 B | Disk registers and buffer |
| `0xF400` | 1 B | Random number generator |
| `0xF401` – `0xFFFF` | 3071 B | Reserved MMIO (not wired yet) |

The bus rejects the reserved regions with `unimplemented!()`.

## Bus
- CPU memory accesses always call into the bus, which in turn calls the matching device `read`/`write`.
- Devices own their buffers; the bus itself does not store data.

## RAM (`crates/mb8/src/dev/ram.rs`)
- Plain byte-addressable memory. Writes update the backing array; reads return what was last written.
- `RAM_SIZE = 0xC000`. The stack grows downward (`STACK_TOP = 0xBFFF`, `STACK_BOTTOM = 0xBF00`).

## ROM (`crates/mb8/src/dev/rom.rs`)
- Backing store for program code (`ROM_SIZE = 0x1000`).
- The device currently accepts writes from the bus, but programs should not rely on mutating ROM; this may be blocked in the future. ROM is meant to hold the kernel/boot image.

## GPU (`crates/mb8/src/dev/gpu.rs`)
- Registers live at `0xF000` (offsets relative to that base):
  - `0x0000` — mode register. `0x00` = off, `0x01` = TTY.
  - `0x0001` — TTY data register. When mode is TTY, each write pushes a character to the screen and advances the cursor.
- Reading `0x0000` returns the current mode. Other reads are currently unimplemented.

## Keyboard (`crates/mb8/src/dev/keyboard.rs`)
- Registers at `0xF100` (offsets relative to that base):
  - `0x00` — `STATUS`. Returns `1` when keys are queued, otherwise `0`.
  - `0x01` — `DATA`. Reading pops the next key code from the queue; returns `0` when empty.
- Writes are ignored.

## Disk (`crates/mb8/src/dev/disk.rs`)
- Registers at `0xF200` (offsets relative to that base):
  - `0x0000` — `BLOCK` number to operate on.
  - `0x0001` — `CMD` (`0x00` no-op, `0x01` read, `0x02` write).
  - `0x0002`–`0x0102` — 256-byte disk buffer used for reads/writes.
- `CMD` operations move data between the internal image and the buffer; buffer reads/writes go directly to the 256-byte window.

## Random Number Generator (`crates/mb8/src/dev/rand.rs`)
- Registers at `0xF400` (offsets relative to that base):
  - `0x00` — `DATA`. Reading returns the next random number in the sequence.
  - Writes to `DATA` are ignored.
