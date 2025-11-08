# MicroBot-8bit

MicroBot-8bit is a simple, 8-bit virtual machine.

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
        <td>FLAGS</td>
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

Virtual machine handles the following opcodes:

<table>
    <tr>
        <th>asm</th>
        <th>opcode</th>
        <th>short description</th>
    </tr>
    <tr>
        <td>NOP</td>
        <td>0x0000</td>
        <td>No operation</td>
    </tr>
    <tr>
        <td>HALT</td>
        <td>0x0100</td>
        <td>Stop the execution</td>
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
        <td>LDI dst value</td>
        <td>0x2AXX</td>
        <td>Load immediate XX value into A register</td>
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
        <td>LD addr</td>
        <td>0x8XXX</td>
        <td>Load data from memory address XXX into R7 register</td>
    </tr>
    <tr>
        <td>ST addr</td>
        <td>0x9XXX</td>
        <td>Store data from R7 register into memory address XXX</td>
    </tr>
</table>
