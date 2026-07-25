# System Calls

System calls live at `0xE500` (`kernel/syscalls.asm`). To invoke one, load the call ID into `R1` and `CALL 0xE500`. Inputs and outputs travel through the registers listed below; all other registers are caller-saved. `R0`/`A` is reserved as scratch for pseudo-instructions and is not part of the syscall ABI.

- **0x01 — SYS_GPU_MODE**<br>
  Input: `R2` mode byte (`0x00` off, `0x01` TTY). Writes the GPU mode register at `0xF000`.

- **0x02 — SYS_WRITE**<br>
  Input: `R2` character byte. Sends it to the GPU TTY data register at `0xF001`.

- **0x03 — SYS_WRITELN**<br>
  Input: `R2:R3` address of a zero-terminated string. Streams characters to the TTY data register until `0x00`.

- **0x04 — SYS_WAIT_FOR_KEY**<br>
  Blocks until the keyboard status register (`0xF101`) is non-zero. No outputs.

- **0x05 — SYS_READ_KEY**<br>
  Output: `R1` key code popped from the keyboard data register (`0xF102`). Returns `0` if the queue was empty.

- **0x06 — SYS_DISK_SET_BLOCK**<br>
  Input: `R2` block index. Stores it in the disk block register at `0xF200` for later operations.

- **0x07 — SYS_DISK_READ_BLOCK**<br>
  Uses the previously selected block and copies it into the disk buffer window (`0xF202`–`0xF302`).

- **0x08 — SYS_DISK_WRITE_BLOCK**<br>
  Flushes the current disk buffer window into the previously selected block.

- **0x09 — SYS_FS_LIST**<br>
  Input: `R2:R3` destination buffer. Copies the directory block (block `0`) from disk into RAM via `MEMCPY`.

- **0x0A — SYS_FS_FIND**<br>
  Input: `R2:R3` filename pointer.<br>
  Output: `R1` status (`0` success, `1` not found), `R2` block index, `R3` file size.

- **0x0B — SYS_FS_READ**<br>
  Input: `R2:R3` filename pointer, `R4:R5` destination buffer.<br>
  Output: `R1` status (`0` success, `1` not found). On success it loads the file into the buffer using the disk buffer window.

- **0x0C — SYS_FS_WRITE**<br>
  Input: `R2:R3` filename pointer. Currently unimplemented.

- **0x0D — SYS_FS_DELETE**<br>
  Input: `R2:R3` filename pointer. Currently unimplemented.

- **0x0E — SYS_EXEC**<br>
  Input: `R2:R3` filename pointer. Loads the file into RAM at `0x1000` (user entry) and jumps to it.<br>
  Output: `R1` status (`0` success, `1` not found).

- **0x0F — SYS_EXIT**<br>
  No inputs. Returns control to the kernel entrypoint at `0xE000` (used by user programs to quit).

- **0x10 — SYS_RAND**<br>
  Output: `R1` random byte.
