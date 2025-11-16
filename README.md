# MicroBot-8bit

MicroBot-8bit is a simple, 8-bit virtual machine.

<img width="752" height="460" alt="Logo" src="https://github.com/user-attachments/assets/972c07bb-a62e-4b11-b184-28fd43031586" />


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
