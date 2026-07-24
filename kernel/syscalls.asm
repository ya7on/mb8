#once

#include "../asm/std.asm"

SYS_GPU_MODE = 0x01
SYS_WRITE = 0x02
SYS_WRITELN = 0x03
SYS_WAIT_FOR_KEY = 0x04
SYS_READ_KEY = 0x05
SYS_DISK_SET_BLOCK = 0x06
SYS_DISK_READ_BLOCK = 0x07
SYS_DISK_WRITE_BLOCK = 0x08
SYS_FS_LIST = 0x09
SYS_FS_FIND = 0x0A
SYS_FS_READ = 0x0B
SYS_FS_WRITE = 0x0C
SYS_FS_DELETE = 0x0D
SYS_EXEC = 0x0E
SYS_EXIT = 0x0F
SYS_RAND = 0x10

#addr 0xE500
K_SYSCALL_ENTRY:

syscall_table:
    CMP R1 SYS_GPU_MODE
    JNZR [.sys_tty_write]
    JMP [sys_gpu_mode]
.sys_tty_write:
    CMP R1 SYS_WRITE
    JNZR [.sys_tty_writeln]
    JMP [sys_tty_write]
.sys_tty_writeln:
    CMP R1 SYS_WRITELN
    JNZR [.sys_wait_for_key]
    JMP [sys_tty_writeln]
.sys_wait_for_key:
    CMP R1 SYS_WAIT_FOR_KEY
    JNZR [.sys_read_key]
    JMP [sys_wait_for_key]
.sys_read_key:
    CMP R1 SYS_READ_KEY
    JNZR [.sys_disk_set_block]
    JMP [sys_read_key]
.sys_disk_set_block:
    CMP R1 SYS_DISK_SET_BLOCK
    JNZR [.sys_disk_read_block]
    JMP [sys_disk_set_block]
.sys_disk_read_block:
    CMP R1 SYS_DISK_READ_BLOCK
    JNZR [.sys_disk_write_block]
    JMP [sys_disk_read_block]
.sys_disk_write_block:
    CMP R1 SYS_DISK_WRITE_BLOCK
    JNZR [.sys_fs_list]
    JMP [sys_disk_write_block]
.sys_fs_list:
    CMP R1 SYS_FS_LIST
    JNZR [.sys_fs_find]
    JMP [sys_fs_list]
.sys_fs_find:
    CMP R1 SYS_FS_FIND
    JNZR [.sys_fs_read]
    JMP [sys_fs_find]
.sys_fs_read:
    CMP R1 SYS_FS_READ
    JNZR [.sys_fs_write]
    JMP [sys_fs_read]
.sys_fs_write:
    CMP R1 SYS_FS_WRITE
    JNZR [.sys_fs_delete]
    JMP [sys_fs_write]
.sys_fs_delete:
    CMP R1 SYS_FS_DELETE
    JNZR [.sys_exec]
    JMP [sys_fs_delete]
.sys_exec:
    CMP R1 SYS_EXEC
    JNZR [.sys_exit]
    JMP [sys_exec]
.sys_exit:
    CMP R1 SYS_EXIT
    JNZR [.sys_rand]
    JMP [sys_exit]
.sys_rand:
    CMP R1 SYS_RAND
    JNZR [.not_found]
    JMP [sys_rand]
.not_found:
    RET

; Sets the GPU mode
;
; Input
; R2: The mode to set
;
; Output
; None
sys_gpu_mode:
    ; Locals
    ; R2 - args
    ; R6:R7 = 0xF000
    LDI R6 0xF0
    LDI R7 0x00
    ST [R6:R7] R2
    RET

; Writes a character to the terminal
;
; Input
; R2: The character to write
;
; Output
; None
sys_tty_write:
    ; Locals
    ; R2 - args
    ; R6:R7 = 0xF001
    LDI R6 0xF0
    LDI R7 0x01
    ST [R6:R7] R2
    RET

; Writes a in-memory string to the terminal
;
; Input
; R2: High address of the string to write
; R3: Low address of the string to write
;
; Output
; None
sys_tty_writeln:
    ; Locals
    ; R2, R3 - args
    ; R5 char
    ; R6:R7 = 0xF001
    LDI R6 0xF0
    LDI R7 0x01
.loop:
    LD R5 [R2:R3]
    CMP R5 0x00
    JZR [.end_loop]
    ST [R6:R7] R5
    INC R3
    JR [.loop]
.end_loop:
    RET

; Waits for a key press
;
; Input
; None
;
; Output
; None
sys_wait_for_key:
    ; Locals
    ; R5 key
    ; R6:R7 = 0xF101
    LDI R6 0xF1
    LDI R7 0x01
.loop:
    LD R5 [R6:R7]
    CMP R5 0x00
    JZR [.loop]
    RET

; Reads a key press
;
; Input
; None
;
; Output
; R1: The key pressed
sys_read_key:
    ; Locals
    ; R1 - return value
    ; R6:R7 = 0xF102
    LDI R6 0xF1
    LDI R7 0x02
    LD R1 [R6:R7]
    RET

; Sets a disk block
;
; Input
; R2: The block to set
;
; Output
; None
sys_disk_set_block:
    ; Locals
    ; R2 - args
    ; R6:R7 = 0xF200
    LDI R6 0xF2
    LDI R7 0x00
    ST [R6:R7] R2
    RET

; Reads a disk block into the disk buffer
;
; Input
; None
;
; Output
; None
sys_disk_read_block:
    ; Locals
    ; R0 - DISK_CMD_READ
    ; R6:R7 = 0xF201
    LDI R6 0xF2
    LDI R7 0x01
    LDI R0 0x01
    ST [R6:R7] R0
    RET

; Writes a disk buffer into the disk
;
; Input
; None
;
; Output
; None
sys_disk_write_block:
    ; Locals
    ; R0 - DISK_CMD_WRITE
    ; R6:R7 = 0xF201
    LDI R6 0xF2
    LDI R7 0x01
    LDI R0 0x02
    ST [R6:R7] R0
    RET

; Writes a directory block into the memory
;
; Input
; R2: High address of the directory block to write
; R3: Low address of the directory block to write
;
; Output
; None
sys_fs_list:
    MOV R4 R3 ; R3 -> R4
    MOV R3 R2 ; R2 -> R3
    ; Locals
    ; R2 - 0 disk block | index
    ; R3, R4 - args
    ; R5:R6 = 0xF202
    ; R7 - 255 const. block size | step

    ; Prepare disk block
    LDI R2 0x00
    CALL [sys_disk_set_block]
    CALL [sys_disk_read_block]
    ; Set disk buffer address
    LDI R5 0xF2
    LDI R6 0x02
    LDI R7 0xFF

    MEMCPY [R3:R4] [R5:R6] R7
    RET

; Finds a file in the FS
;
; Input
; R2: High address of the filename to find
; R3: Low address of the filename to find
;
; Output
; R1 - status (0 = success, 1 = not found)
; R2 - block index
; R3 - size
sys_fs_find:
    ; Normalize the shifted public ABI for the existing internal layout.
    MOV R1 R2
    MOV R2 R3

    ; Get 0 block
    MOV R3 R1
    MOV R4 R2

    LDI R2 0x00
    CALL [sys_disk_set_block]
    CALL [sys_disk_read_block]

    MOV R1 R3
    MOV R2 R4

    ; Locals
    ; R1:R2 - args
    ; R3 - file index
    ; R4:R5 buffer ptr
    ; R6 - byte

    LDI R3 0x00 ; file index
    LDI R4 0xF2 ; buffer ptr high
    LDI R5 0x02 ; buffer ptr low
.file:
    CMP R3 0x10
    JNZR [.load_byte]
    JMP [.not_found]

.load_byte:
    LD R6 [R4:R5]
    CMP R6 0x00
    JNZR [.metadata]
    JMP [.next_file]

.metadata:
    INC16 R4:R5 ; start_block
    LD R0 [R4:R5]
    PUSH R0
    INC16 R4:R5 ; size
    LD R0 [R4:R5]
    PUSH R0
    INC16 R4:R5 ; filename

    PUSH R1
    PUSH R2
    PUSH R4
    PUSH R5
    STRCMP R7 R6 R1 R2 R4 R5
    POP R5
    POP R4
    POP R2
    POP R1
    CMP R7 0x00
    JZR [.success]
    POP R0
    POP R0
.next_file:
    INC R3
    LDI R7 0x10
    MUL R6 R7 R3
    LDI R4 0xF2 ; buffer ptr high
    LDI R5 0x02 ; buffer ptr low
.iter:
    INC16 R4:R5
    DEC R6
    CMP R6 0x00
    JNZR [.iter]
    JMP [.file]
.not_found:
    LDI R1 0x01
    RET
.success:
    POP R3
    POP R2
    LDI R1 0x00
    RET

; Finds a file in the FS
;
; Input
; R2: High address of the filename to find
; R3: Low address of the filename to find
; R4: High address of the buffer to write to
; R5: Low address of the buffer to write to
;
; Output
; R1 - status (0 = success, 1 = not found)
sys_fs_read:
    PUSH R4
    PUSH R5
    CALL [sys_fs_find]
    POP R5
    POP R4

    CMP R1 0x00
    JNZR [.not_found]
.copy_block:
    CALL [sys_disk_set_block]
    CALL [sys_disk_read_block]

    LDI R6 0xF2
    LDI R7 0x02

    LDI R1 0x00
.copy_byte:
    LD R0 [R6:R7]
    ST [R4:R5] R0

    LDI R0 0x01
    ADD R7 R0
    CMP R7 0x00
    JNZR [.no_carry_buf]
    INC R6
.no_carry_buf:
    INC R5
    CMP R5 0x00
    JNZR [.no_carry_dst]
    INC R4
.no_carry_dst:
    INC R1
    CMP R1 0x00
    JNZR [.copy_byte]

    DEC R3
    CMP R3 0x00
    JZR [.eof]

    INC R2
    JR  [.copy_block]
.eof:
    LDI R1 0x00
    RET
.not_found:
    LDI R1 0x01
    RET

; Input
; R2:R3 - filename pointer
sys_fs_write:
    RET

; Input
; R2:R3 - filename pointer
sys_fs_delete:
    RET

; Executes a file in the FS
;
; Input
; R2: High address of the filename to find
; R3: Low address of the filename to find
;
; Output
; R1 - status (0 = success, 1 = not found)
sys_exec:
    LDI R1 SYS_FS_READ
    LDI R4 0x10
    LDI R5 0x00
    CALL [K_SYSCALL_ENTRY]
    CMP R1 0x00
    JNZR [.error]

    POP R0
    POP R0

    JMP [0x1000]

.error:
    RET

sys_exit:
    POP R0
    POP R0
    JMP [0xE100]
    RET

; Returns a random byte from MMIO RNG
;
; Input:
; R1: sys_rand
;
; Output:
; R1: Data - Random Byte
sys_rand:
    LDI R6 0xF4
    LDI R7 0x00
    LD R1 [R6:R7]
    RET
