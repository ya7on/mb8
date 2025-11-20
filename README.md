# MicroBot-8bit

[![GitHub License](https://img.shields.io/github/license/ya7on/mb8)](LICENSE)
[![codecov](https://codecov.io/gh/ya7on/mb8/graph/badge.svg?token=UCYX4KOI0F)](https://codecov.io/gh/ya7on/mb8)
[![Docs](https://img.shields.io/github/actions/workflow/status/ya7on/mb8/docs.yml?label=docs)](https://ya7on.github.io/mb8)
[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/ya7on/mb8)
[![GitHub top language](https://img.shields.io/github/languages/top/ya7on/mb8)](https://github.com/ya7on/mb8)
[![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/ya7on/mb8)](https://github.com/ya7on/mb8)

MicroBot-8bit is a simple, 8-bit virtual machine.

<img width="752" height="620" alt="Shell" src="https://github.com/user-attachments/assets/ca54e6c4-da30-4d46-94ef-6aabdbacee51" />

# Running

## Compile assembly

We are using the `customasm` tool to compile assembly code into MB8 machine instructions.

First, install the tool by running:

```
cargo install customasm
```

Then, compile an assembly file using the following command:

```
customasm <file.asm>
```

Compiled binary files have the extension `.bin`.

## Run VM

To run the compiled binary, use the following command:

```
cargo run -- run <file.bin>
```

## Assembly

You can see examples of assembly code in the [`examples`](examples) directory.

- [`bouncing.asm`](examples/bouncing.asm) - Bouncing example
- [`logo.asm`](examples/logo.asm) - Logo example

You can compile any of them and run them using the following commands:

```
customasm ./examples/logo.asm && cargo run -- run ./examples/logo.bin
```

# Architecture

Full documentation can be found in the [`book`](https://ya7on.github.io/mb8/).
