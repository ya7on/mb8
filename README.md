# MicroBot-8bit

[![GitHub License](https://img.shields.io/github/license/ya7on/mb8)](LICENSE)
[![codecov](https://codecov.io/gh/ya7on/mb8/graph/badge.svg?token=UCYX4KOI0F)](https://codecov.io/gh/ya7on/mb8)
[![Docs](https://img.shields.io/github/actions/workflow/status/ya7on/mb8/docs.yml?label=docs)](https://ya7on.github.io/mb8)
[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/ya7on/mb8)
[![GitHub top language](https://img.shields.io/github/languages/top/ya7on/mb8)](https://github.com/ya7on/mb8)
[![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/ya7on/mb8)](https://github.com/ya7on/mb8)

MicroBot-8bit is an 8-bit microcomputer in the vein of the ZX Spectrum/Commodore 64, originally inspired by CHIP-8. It comes with a tiny CP/M-like OS layer, a GPU TTY, keyboard input, and a disk-backed filesystem stub.

<img width="752" height="620" alt="sh.bin" src="https://github.com/user-attachments/assets/b42f6d26-9517-4a04-8582-4d56fb8cd0d7" />

# Running

## Compile assembly

We use [`customasm`](https://github.com/hlorenzi/customasm) to build all assembly sources.

Install once:
```
cargo install customasm
```

Build everything (kernel, user-space programs, tests):
```
make all
```
Targets:
- `make kernel` — build the kernel image (`kernel/main.bin`)
- `make user` — build user-space programs under `user/`
- `make tests` — build assembly tests under `kernel/tests`

## Run VM

After building, run the VM with the kernel entrypoint first and then any user programs:
```
make run
```
The first path is always the kernel; subsequent arguments are user-space binaries loaded by the OS.

## Assembly

User-space programs live under `user/`. For a minimal shell example, see `user/sh.asm`; build with `make user` and run with the kernel:
```
make run
```

# Architecture

Full documentation can be found in the [`book`](https://ya7on.github.io/mb8/).
