# Assembler diagnostics

This page lists every diagnostic currently emitted by the MB8 assembler. The diagnostic code in
the command-line output links directly to the corresponding section below.

## A0100

**Lex Error**

The assembler found a character, malformed hexadecimal number, string escape, or unterminated
string that cannot be converted into a token.

**Example**

```asm
LDI R1, 42
```

Decimal literals are not part of the assembly syntax.

**How to fix**

Use a hexadecimal literal and make sure strings contain only supported escapes.

```asm
LDI R1, 0x2A
```

## A0101

**Parse Error**

The tokens are valid individually, but they do not form a valid instruction, label, or directive.
This commonly means that punctuation or an operand is missing.

**Example**

```asm
LDI R1 0x2A
```

**How to fix**

Follow the required instruction form, including the comma between operands.

```asm
LDI R1, 0x2A
```

## A0200

**Include Error**

The assembler could not read or expand a file named by an `.include` directive. The diagnostic
message contains the underlying file or include error.

**Example**

```asm
.include "missing.asm"
```

**How to fix**

Correct the path, create the included file, or fix its permissions. Include paths are resolved
relative to the file containing the directive.

## A0300

**Unsupported Instruction Form**

The mnemonic and operands do not match any core instruction or pseudo-instruction known to the
assembler.

**Example**

```asm
MOV R1, 0x01
```

**How to fix**

Use a supported operand form. For example, load an immediate value with `LDI`.

```asm
LDI R1, 0x01
```

## A0301

**Duplicate Label**

The same label is defined more than once in its scope. The diagnostic marks both definitions.

**Example**

```asm
loop:
    NOP
loop:
    HALT
```

**How to fix**

Rename or remove one definition, and update any references to the renamed label.

```asm
loop:
    NOP
done:
    HALT
```

## A0302

**Unknown Label**

An instruction refers to a label that is not defined in the applicable source scope.

**Example**

```asm
JMP missing
```

**How to fix**

Define the label or correct its spelling.

```asm
JMP done

done:
    HALT
```

## A0303

**Unexpected Directive After Include Expansion**

An `.include` or `.const` directive reached an internal assembler stage where it should already
have been expanded or removed. This indicates an assembler bug, not an error that assembly source
is normally expected to cause.

**How to fix**

Please report the bug and include the source files, assembler version, and full diagnostic output.

## A0304

**Duplicate Origin Directive**

An assembled program contains more than one `.origin` directive. The diagnostic marks the first
and duplicate directives.

**Example**

```asm
.origin 0x1000
.origin 0x2000
```

**How to fix**

Keep a single `.origin` directive in the root source.

```asm
.origin 0x1000
```

## A0305

**Address Overflow**

Emitting an instruction or data would advance the current address beyond `0xFFFF`, the largest
address representable by the assembler.

**Example**

```asm
.origin 0xFFFF
NOP
```

**How to fix**

Move the origin to a lower address or reduce the amount of emitted code or data.

```asm
.origin 0xFFFD
NOP
```

## A0306

**Value Out of Range**

An immediate value does not fit the operand width required by the selected instruction form.

**Example**

```asm
LDI R1, 0x0100
```

**How to fix**

Use a value within the stated range or an instruction form that accepts the wider value.

```asm
LDI R1, 0xFF
```

## A0307

**Relative Jump Out of Range**

The target of a relative jump is too far from the jump instruction to fit in a signed 8-bit
offset.

**Example**

```asm
start:
    JR [far]
    .addr 0x0200
far:
    HALT
```

**How to fix**

Move the target closer or use an absolute jump.

```asm
JMP far
```

## A0308

**Scratch Register Conflict**

A pseudo-instruction needs a scratch register that is also used as one of its operands. Expanding
the instruction would overwrite that operand.

**Example**

```asm
MUL A, R1, R2
```

**How to fix**

Use a different operand register, or replace the pseudo-instruction with explicit core
instructions that preserve the value.

```asm
MUL R3, R1, R2
```

## A0309

**Invalid Address Directive**

An `.addr` directive targets an address below the current address. The assembler can pad forward,
but it cannot move backward or overwrite bytes already emitted.

**Example**

```asm
.origin 0x1000
NOP
.addr 0x1001
```

**How to fix**

Choose a target at or above the current address.

```asm
.origin 0x1000
NOP
.addr 0x1002
```

## A0310

**Duplicate Constant**

The same constant name is defined more than once. The diagnostic marks both definitions.

**Example**

```asm
.const @COLOR, 0x01
.const @COLOR, 0x02
```

**How to fix**

Keep one definition or give the constants distinct names.

```asm
.const @FOREGROUND, 0x01
.const @BACKGROUND, 0x02
```

## A0311

**Unknown Constant**

An operand refers to a constant that has not been defined.

**Example**

```asm
LDI R1, @COLOR
```

**How to fix**

Define the constant or correct its spelling.

```asm
.const @COLOR, 0x01
LDI R1, @COLOR
```
