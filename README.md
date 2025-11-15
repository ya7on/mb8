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
cargo run <file.bin>
```

## Assembly

You can see examples of assembly code in the [`examples`](examples) directory.

- [`bouncing.asm`](examples/bouncing.asm) - Bouncing example
- [`logo.asm`](examples/logo.asm) - Logo example

You can compile any of them and run them using the following commands:

```
customasm ./examples/logo.asm && cargo run ./examples/logo.bin
```

# Architecture

## Registers

Virtual machine contains the following registers:

<table>
    <tr>
        <th>register</th>
        <th>mask</th>
        <th>description</th>
    </tr>
    <tr>
        <td>R0 - R7</td>
        <td>0x00 - 0x07</td>
        <td>General purpose registers</td>
    </tr>
    <tr>
        <td>I</td>
        <td>0x0C</td>
        <td>Index register</td>
    </tr>
    <tr>
        <td>SP</td>
        <td>0x0D</td>
        <td>Stack pointer</td>
    </tr>
    <tr>
        <td>PC</td>
        <td>0x0E</td>
        <td>Program counter</td>
    </tr>
    <tr>
        <td>F</td>
        <td>0x0F</td>
        <td>Flags register</td>
    </tr>
</table>

Registers 0x08 - 0x0C are reserved for future use.

## Flags

Virtual machine handles the following flags:

<table>
    <tr>
        <th>flag</th>
        <th>mask</th>
        <th>description</th>
    </tr>
    <tr>
        <td>Z</td>
        <td>0x01</td>
        <td>Zero flag. If the result of an operation is zero, this flag is set.</td>
    </tr>
    <tr>
        <td>N</td>
        <td>0x02</td>
        <td>Negative flag. If the result of an operation is negative, this flag is set.</td>
    </tr>
    <tr>
        <td>C</td>
        <td>0x04</td>
        <td>Carry flag. If the result of an operation is greater than 255, this flag is set. If the result of an operation is less than 0, this flag is set.</td>
    </tr>
</table>

Flags 0x08, 0x10, 0x20, 0x40, 0x80 are reserved for future use.

## Opcodes

An instruction for mb8 is 16 bits wide, represented as 0xABCD, where:
- A — the most significant nibble (4 bits) — instruction group
- B — sub-opcode, register, or flag, depending on the group
- C — typically a register or the upper 4 bits of an address
- D — typically a register or the lower 4 bits of an address

For example, the binary representation of the instruction `ADD R0, R1` is `0x1101`:
```
0001 0001 0000 0001
```

In branch/load/store operations, the address is 12 bits (XXX in 0xYXXX), similar to CHIP-8, allowing addressing up to 4 KiB of memory.

Virtual machine handles the following opcodes:

<table>
    <tr>
        <th>asm</th>
        <th>opcode</th>
        <th>short description</th>
    </tr>
    <tr>
        <td colspan="3">0x0 GROUP: System operations</td>
    </tr>
    <tr>
        <td>NOP</td>
        <td>0x0000</td>
        <td>No operation</td>
    </tr>
    <tr>
        <td>HALT</td>
        <td>0x01XX</td>
        <td>Stop the execution (in future XX exit code will be added)</td>
    </tr>
    <tr>
        <td>PUTC src</td>
        <td>0x020A</td>
        <td>Put the character from the A register into the terminal output</td>
    </tr>
    <tr>
        <td colspan="3">0x1 REG-REG GROUP: Operations with two registers. ALU operations</td>
    </tr>
    <tr>
        <td>MOV dst src</td>
        <td>0x10AB</td>
        <td>Move data from B register to A register</td>
    </tr>
    <tr>
        <td>ADD dst src</td>
        <td>0x11AB</td>
        <td>Put the sum of A and B registers into A register</td>
    </tr>
    <tr>
        <td>SUB dst src</td>
        <td>0x12AB</td>
        <td>Put the difference of A and B registers into A register</td>
    </tr>
    <tr>
        <td>AND dst src</td>
        <td>0x13AB</td>
        <td>Put the result of logical AND of A and B registers into A register</td>
    </tr>
    <tr>
        <td>OR dst src</td>
        <td>0x14AB</td>
        <td>Put the result of logical OR of A and B registers into A register</td>
    </tr>
    <tr>
        <td>XOR dst src</td>
        <td>0x15AB</td>
        <td>Put the result of logical XOR of A and B registers into A register</td>
    </tr>
    <tr>
        <td>SHR dst</td>
        <td>0x16AB</td>
        <td>Shift A register right by count of bits stored in B register</td>
    </tr>
    <tr>
        <td>SHL dst</td>
        <td>0x17AB</td>
        <td>Shift A register left by count of bits stored in B register</td>
    </tr>
    <tr>
        <td colspan="3">0x2 LDI</td>
    </tr>
    <tr>
        <td>LDI dst value</td>
        <td>0x2AXX</td>
        <td>Load immediate XX value into A register</td>
    </tr>
    <tr>
        <td colspan="3">0x3 - 0x5 JUMP GROUP: Jump instructions</td>
    </tr>
    <tr>
        <td>JMP addr</td>
        <td>0x3XXX</td>
        <td>Jump to XXX address</td>
    </tr>
    <tr>
        <td>JZ addr</td>
        <td>0x4XXX</td>
        <td>Jump to XXX address if Flag register has zero flag</td>
    </tr>
    <tr>
        <td>JNZ addr</td>
        <td>0x5XXX</td>
        <td>Jump to XXX address if Flag register does not have zero flag</td>
    </tr>
    <tr>
        <td colspan="3">0x6 - 0x7 STACK: Stack and subroutine operations</td>
    </tr>
    <tr>
        <td>CALL addr</td>
        <td>0x6XXX</td>
        <td>Call subroutine at XXX address</td>
    </tr>
    <tr>
        <td>RET</td>
        <td>0x7000</td>
        <td>Return from subroutine</td>
    </tr>
    <tr>
        <td>PUSH dst</td>
        <td>0x71A0</td>
        <td>Push data from A register onto stack</td>
    </tr>
    <tr>
        <td>POP src</td>
        <td>0x72A0</td>
        <td>Pop data from stack into A register</td>
    </tr>
    <tr>
        <td colspan="3">0x8 - 0x9 MEMORY: Memory operations</td>
    </tr>
    <tr>
        <td>LDI_I addr</td>
        <td>0x8XXX</td>
        <td>Load address XXX into I register</td>
    </tr>
    <tr>
        <td>LD src</td>
        <td>0x90A0</td>
        <td>Load data from memory address stored in I register into A register</td>
    </tr>
    <tr>
        <td>ST src</td>
        <td>0x91A0</td>
        <td>Store data from A register into memory address stored in I register</td>
    </tr>
    <tr>
        <td>INC_I src</td>
        <td>0x92A0</td>
        <td>Increment memory address stored in I register by value in A register</td>
    </tr>
    <tr>
        <td>DEC_I src</td>
        <td>0x93A0</td>
        <td>Decrement memory address stored in I register by value in A register</td>
    </tr>
    <tr>
        <td colspan="3">0xA DRAWING: Drawing operations</td>
    </tr>
    <tr>
        <td>DRAW x y height</td>
        <td>0xA000</td>
        <td>Draw sprite at (x, y) with height height. Sprite is stored in memory starting at address stored in I register. Sprite is drawn using XOR operation. If sprite overlaps with existing pixels, it will invert the color of those pixels.</td>
    </tr>
</table>

## Stack

Stack size in mb8 VM is the first 256 bytes in the beginning of the memory.
