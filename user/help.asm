.origin 0x1000

start:
    LDI R1, 0x03
    LDI R2:R3, HELP_TEXT
    CALL [0xE500]

    LDI R1, 0x0F
    CALL [0xE500]

HELP_TEXT:
    .ascii "MB8 - 8bit fantasy computer\n"
    .ascii "\n"
    .ascii "Commands:\n"
    .ascii "help  - Display help information\n"
    .ascii "ls    - List files\n"
    .ascii "exit  - Exit the system\n\0"

.addr 0x2000
