# Register set

Small and simple; each VM context (judge or bot) gets its own copies.

<table>
    <tr>
        <th>register</th>
        <th>mask</th>
        <th>description</th>
        <th>size</th>
    </tr>
    <tr>
        <td>R0 - R7</td>
        <td>0x00 - 0x07</td>
        <td>General-purpose registers</td>
        <td>8 bits</td>
    </tr>
    <tr>
        <td>SP</td>
        <td>0x0D</td>
        <td>Stack pointer; PUSH / POP move it upward</td>
        <td>8 bits</td>
    </tr>
    <tr>
        <td>PC</td>
        <td>0x0E</td>
        <td>Program counter; steps by 2 bytes per instruction</td>
        <td>12 bits</td>
    </tr>
    <tr>
        <td>F</td>
        <td>0x0F</td>
        <td>Flags register (Z/N/C)</td>
        <td>8 bits</td>
    </tr>
</table>

Notes:
- `F` is overwritten by arithmetic/logic/shift ops. Jumps read it, but most other ops leave it untouched.
- Context switches keep register sets separate: judge and each bot maintain their own registers (`PC`, `SP`, `R0-R7`).
- Masks `0x08 - 0x0C` are reserved for future expansion.
