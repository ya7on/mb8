# Memory model

Each context (judge or bot) has a separate code ROM and data RAM bank.

## ROM

- Read-only, 4096 bytes.
- Holds 16-bit instructions only; no self-modifying code.
- Jumps/Calls target ROM addresses; PC wraps at the end of the bank and halts.

## RAM

- Read-write, 4096 bytes, private per context.
- Layout:
  - `0x0000..0x00FF` — **stack** (256 bytes), `SP` points to the next free byte. `PUSH`/`POP` grow upward.
  - `0x0100..0x0EFF` — **general-purpose data**; use `I` + `LD/ST/INC_I/DEC_I` to access.
  - `0x0F00..0x0FFF` — **special region**:
    - For the judge: graphic buffer used by `DRAW` to paint to the screen.
    - For bots: mailbox shared with the judge/other bots, accessed via `LDG`/`STG`.
- Bots and the judge do not see each other’s RAM directly; interaction happens only through mailboxes and explicit context switches.
