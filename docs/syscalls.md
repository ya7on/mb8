# System Calls

System calls live at `0xE500` (`kernel/syscalls.asm`). To invoke one, load the call ID into `R0` and `CALL 0xE500`. Inputs and outputs travel through the registers listed below; all other registers are caller-saved.

- **0x01 — SYS_GPU_MODE**  
  Input: `R1` mode byte (`0x00` off, `0x01` TTY). Writes the GPU mode register at `0xF000`.

- **0x02 — SYS_WRITE**  
  Input: `R1` character byte. Sends it to the GPU TTY data register at `0xF001`.

- **0x03 — SYS_WRITELN**  
  Input: `R1:R2` address of a zero-terminated string. Streams characters to the TTY data register until `0x00`.

- **0x04 — SYS_WAIT_FOR_KEY**  
  Blocks until the keyboard status register (`0xF101`) is non-zero. No outputs.

- **0x05 — SYS_READ_KEY**  
  Output: `R0` key code popped from the keyboard data register (`0xF101`). Returns `0` if the queue was empty.

- **0x06 — SYS_DISK_SET_BLOCK**  
  Input: `R1` block index. Stores it in the disk block register at `0xF200` for later operations.

- **0x07 — SYS_DISK_READ_BLOCK**  
  Uses the previously selected block and copies it into the disk buffer window (`0xF202`–`0xF302`).

- **0x08 — SYS_DISK_WRITE_BLOCK**  
  Flushes the current disk buffer window into the previously selected block.

- **0x09 — SYS_FS_LIST**  
  Input: `R1:R2` destination buffer. Copies the directory block (block `0`) from disk into RAM via `MEMCPY`.

- **0x0A — SYS_FS_FIND**  
  Input: `R1:R2` filename pointer.  
  Output: `R0` status (`0` success, `1` not found), `R1` block index, `R2` file size.

- **0x0B — SYS_FS_READ**  
  Input: `R1:R2` filename pointer, `R3:R4` destination buffer.  
  Output: `R0` status (`0` success, `1` not found). On success it loads the file into the buffer using the disk buffer window.

- **0x0C — SYS_FS_WRITE**  
  Currently unimplemented placeholder.

- **0x0D — SYS_FS_DELETE**  
  Currently unimplemented placeholder.

- **0x0E — SYS_EXEC**  
  Input: `R1:R2` filename pointer. Loads the file into RAM at `0x1000` (user entry) and jumps to it.  
  Output: `R0` status (`0` success, `1` not found).

- **0x0F — SYS_EXIT**  
  No inputs. Returns control to the kernel entrypoint at `0xE000` (used by user programs to quit).
