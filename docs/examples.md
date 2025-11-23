# Examples

User-space programs now live under `user/`. A good starting point is the shell at `user/sh.asm`: the kernel loads it into RAM at `0x1000` and jumps to it after boot. Build it with `make user` and run via `cargo run -- run ./kernel/main.bin ./user/sh.bin ./user/hw.bin ./user/ls.bin`.
