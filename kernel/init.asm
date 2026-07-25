.addr 0xE100
START_SHELL:
    LDI R1, @SYS_EXEC
    LDI R2:R3, SHELL_BIN
    CALL [K_SYSCALL_ENTRY]

    LDI R1, @SYS_WRITELN
    LDI R2:R3, SHELL_NOT_FOUND
    CALL [K_SYSCALL_ENTRY]

panic:
    JR [panic]

_error:
    HALT

MB8_BANNER:
    .ascii "MB8 kernel is starting...\n\0"

LOADING:
    .ascii "Type 'help' for more information\n\0"

SHELL_BIN:
    .ascii "sh\0"

SHELL_NOT_FOUND:
    .ascii "KERNEL PANIC: shell executable not found\n\0"
