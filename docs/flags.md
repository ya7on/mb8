# Flags

These live in the `F` register and are rewritten by arithmetic/logic/shift instructions:

<table>
    <tr>
        <th>flag</th>
        <th>mask</th>
        <th>description</th>
        <th>set by</th>
    </tr>
    <tr>
        <td>Z</td>
        <td>0x01</td>
        <td>Result is zero.</td>
        <td>ADD, SUB, AND, OR, XOR, SHL, SHR (and pseudo-instructions that expand to them)</td>
    </tr>
    <tr>
        <td>N</td>
        <td>0x02</td>
        <td>Copies bit 7 (sign) of the 8-bit result.</td>
        <td>ADD, SUB, AND, OR, XOR, SHL, SHR</td>
    </tr>
    <tr>
        <td>C</td>
        <td>0x04</td>
        <td>Set when an 8-bit result wraps: carry on ADD/SHL/SHR, borrow on SUB.</td>
        <td>ADD, SUB, SHL, SHR</td>
    </tr>
</table>

Notes:
- Instructions not listed leave flags unchanged.
- Pseudo-instructions (`INC`, `DEC`, `CMP`, `CMPI`, shifts) inherit flag behavior from the underlying ops.
- Flags 0x08, 0x10, 0x20, 0x40, 0x80 are reserved for future use.
