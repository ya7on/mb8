# Overview

MB8 is an 8-bit microcomputer in the spirit of the ZX Spectrum and Commodore 64, initially inspired by CHIP-8. It ships with a tiny CP/M-like operating system layer and a minimal assembly-first toolchain.

## What’s inside
- 8-bit CPU with a compact ISA and pseudo-instructions for convenience.
- Memory-mapped devices (RAM, ROM, GPU TTY, keyboard, disk) wired through a simple bus.
- A small kernel plus user-space programs, all written in assembly.

## Running the project
1) Build all assembly artifacts (kernel, user programs, tests):
```sh
make all
```
2) Run the VM, passing the kernel entrypoint first and then any user-space programs:
```sh
cargo run -- run ./kernel/main.bin ./user/sh.bin ./user/hw.bin ./user/ls.bin
```

The kernel image is loaded at `0xE000`, user programs are passed as extra binaries, and the OS provides basic CP/M-like services via syscalls.
