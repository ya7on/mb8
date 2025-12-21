#include "../asm/cpu.asm"
#include "../asm/ext.asm"

main:
.main_0:
	LDI R3 0x1
	ST [0xE] R3
	LDI R2 0x2
	ST [0xF] R2
	LD R1 [0xE]
	ST [0x0] R1
	LD R0 [0xF]
	ST [0x1] R0
	CALL [add]
	MOV R2 R0

	LDI R0 0x0F
    CALL [0xE500]
add:
.add_0:
	LD R0 [0x0]
	ST [0xB] R0
	LD R0 [0x1]
	ST [0xC] R0
	LD R3 [0xB]
	LD R2 [0xC]
	MOV R1 R3
	ADD R1 R2
	MOV R0 R1
	RET
