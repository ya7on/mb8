SYS_GPU_MODE = 0x01
SYS_WRITE = 0x02
SYS_WRITELN = 0x03
SYS_WAIT_FOR_KEY = 0x04
SYS_READ_KEY = 0x05
SYS_DISK_SET_BLOCK = 0x06
SYS_DISK_READ_BLOCK = 0x07
SYS_DISK_WRITE_BLOCK = 0x08

K_SYSCALL_ENTRY:

syscall_table:
    CMPI R0 SYS_GPU_MODE
    JZR sys_gpu_mode
    CMPI R0 SYS_WRITE
    JZR sys_tty_write
    CMPI R0 SYS_WRITELN
    JZR sys_tty_writeln
    CMPI R0 SYS_WAIT_FOR_KEY
    JZR sys_wait_for_key
    CMPI R0 SYS_READ_KEY
    JZR sys_read_key
    CMPI R0 SYS_DISK_SET_BLOCK
    JZR sys_disk_set_block
    CMPI R0 SYS_DISK_READ_BLOCK
    JZR sys_disk_read_block
    CMPI R0 SYS_DISK_WRITE_BLOCK
    JZR sys_disk_write_block
    RET

sys_gpu_mode:
    LDI R6 0xF0
    LDI R7 0x00
    ST R1 R6 R7
    RET

sys_tty_write:
    LDI R6 0xF0
    LDI R7 0x01
    ST R1 R6 R7
    RET

sys_tty_writeln:
    LDI R6 0xF0
    LDI R7 0x01
.loop:
    LD R5 R1 R2
    CMPI R5 0x00
    JZR .end_loop
    ST R5 R6 R7
    INC R2 1
    JR .loop
.end_loop:
    RET

sys_wait_for_key:
    LDI R6 0xF1
    LDI R7 0x00
.loop:
    LD R5 R6 R7
    CMPI R5 0x00
    JZR .loop
    RET

sys_read_key:
    LDI R6 0xF1
    LDI R7 0x01
    LD R0 R6 R7
    RET

sys_disk_set_block:
    LDI R6 0xF2
    LDI R7 0x00
    ST R1 R6 R7
    RET

sys_disk_read_block:
    LDI R6 0xF2
    LDI R7 0x01
    LDI R5 0x01
    ST R5 R6 R7
    RET

sys_disk_write_block:
    LDI R6 0xF2
    LDI R7 0x01
    LDI R5 0x02
    ST R5 R6 R7
    RET
