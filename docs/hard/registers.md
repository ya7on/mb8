# Register set

16 registers, each 8 bits wide. Some are paired into 16-bit pointers through aliases.

<table>
    <tr>
        <th>register</th>
        <th>alias</th>
        <th>mask</th>
        <th>description</th>
        <th>size</th>
    </tr>
    <tr>
        <td>R0</td>
        <td>A</td>
        <td>0x00</td>
        <td>Accumulator / general-purpose</td>
        <td>8 bits</td>
    </tr>
    <tr>
        <td>R1</td>
        <td>-</td>
        <td>0x01</td>
        <td>General-purpose</td>
        <td>8 bits</td>
    </tr>
    <tr>
        <td>R2</td>
        <td>-</td>
        <td>0x02</td>
        <td>General-purpose</td>
        <td>8 bits</td>
    </tr>
    <tr>
        <td>R3</td>
        <td>-</td>
        <td>0x03</td>
        <td>General-purpose</td>
        <td>8 bits</td>
    </tr>
    <tr>
        <td>R4</td>
        <td>-</td>
        <td>0x04</td>
        <td>General-purpose</td>
        <td>8 bits</td>
    </tr>
    <tr>
        <td>R5</td>
        <td>-</td>
        <td>0x05</td>
        <td>General-purpose</td>
        <td>8 bits</td>
    </tr>
    <tr>
        <td>R6</td>
        <td>-</td>
        <td>0x06</td>
        <td>General-purpose</td>
        <td>8 bits</td>
    </tr>
    <tr>
        <td>R7</td>
        <td>-</td>
        <td>0x07</td>
        <td>General-purpose</td>
        <td>8 bits</td>
    </tr>
    <tr>
        <td>R8</td>
        <td>-</td>
        <td>0x08</td>
        <td>General-purpose</td>
        <td>8 bits</td>
    </tr>
    <tr>
        <td>R9</td>
        <td>IH</td>
        <td>0x09</td>
        <td>Index register high byte</td>
        <td>8 bits</td>
    </tr>
    <tr>
        <td>R10</td>
        <td>IL</td>
        <td>0x0A</td>
        <td>Index register low byte</td>
        <td>8 bits</td>
    </tr>
    <tr>
        <td>R11</td>
        <td>FPH</td>
        <td>0x0B</td>
        <td>Frame pointer high byte</td>
        <td>8 bits</td>
    </tr>
    <tr>
        <td>R12</td>
        <td>FPL</td>
        <td>0x0C</td>
        <td>Frame pointer low byte</td>
        <td>8 bits</td>
    </tr>
    <tr>
        <td>R13</td>
        <td>SPH</td>
        <td>0x0D</td>
        <td>Stack pointer high byte</td>
        <td>8 bits</td>
    </tr>
    <tr>
        <td>R14</td>
        <td>SPL</td>
        <td>0x0E</td>
        <td>Stack pointer low byte</td>
        <td>8 bits</td>
    </tr>
    <tr>
        <td>R15</td>
        <td>F</td>
        <td>0x0F</td>
        <td>Flags register (Z/N/C)</td>
        <td>8 bits</td>
    </tr>
</table>

Notes:
- `IH:IL` form a 16-bit index pointer.
- `FPH:FPL` hold the 16-bit frame pointer.
- `SPH:SPL` hold the 16-bit stack pointer; PUSH/POP move it downward.
- `F` is overwritten by arithmetic/logic/shift ops. Jumps read it, other ops leave it untouched.
- Context switches keep register sets separate for each VM context.
