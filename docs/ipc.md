# Judge and bots

The VM runs one judge program and up to four bots in parallel contexts. Each bot gets its own ROM/RAM plus a 256-byte mailbox; the judge keeps a graphic buffer in the same high memory range.

- **Roles**: the judge owns the display and rule logic. Bots are isolated players; they cannot touch the judge’s RAM directly.
- **Switching**: the judge calls `YIELD rX` with a bot id in `rX` to give it CPU time. Bots call `YIELD` to return to the judge. Each context keeps its own registers and RAM.
- **Mailboxes** (bots): write with `STG src botIdReg` and read with `LDG dst botIdReg`. When running inside a bot, omit the id to use its own mailbox.
- **Graphics** (judge): draw via `DRAW`; the graphic buffer lives in the judge’s high RAM. Bots do not draw—talk to the judge via mailboxes instead.
- **CLI flow**: build binaries with `customasm`, then run `cargo run -- run judge.bin --bot bot.bin` to load the judge plus one bot (multiple `--bot` isn’t wired yet).

Keep bot code defensive—the judge controls when and how often bots get scheduled.
