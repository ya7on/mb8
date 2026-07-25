use std::path::Path;

use mb8::vm::VirtualMachine;

fn assemble_kernel_test(body: &str) -> Vec<u8> {
    let source =
        format!(".origin 0xE000\n\n{body}\n\n.include \"kernel/syscalls.asm\"\n\n.addr 0xF000\n");
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let compilation = asm::compile_source(&source, "<mb8-test>".to_string(), &workspace);

    assert!(
        compilation.diagnostics.is_empty(),
        "assembly diagnostics: {:#?}",
        compilation.diagnostics
    );
    let Some(binary) = compilation.result else {
        panic!("assembly produced no binary");
    };
    assert_eq!(binary.len(), 4096, "test ROM must be exactly 4 KiB");
    binary
}

#[test]
fn test_sys_gpu_mode() {
    let bin = assemble_kernel_test(
        r"
start:
    LDI R1, @SYS_GPU_MODE
    LDI R2, 0x01
    CALL [K_SYSCALL_ENTRY]
    HALT
",
    );
    let mut vm = VirtualMachine::default();
    vm.load_rom(&bin);
    vm.run();
    assert_eq!(vm.devices.read(0xF000), 0x01);
}

#[test]
fn test_sys_write() {
    let bin = assemble_kernel_test(
        r"
start:
    LDI R1, @SYS_GPU_MODE
    LDI R2, 0x01
    CALL [K_SYSCALL_ENTRY]

    LDI R1, @SYS_WRITE
    LDI R2, 0x31
    CALL [K_SYSCALL_ENTRY]

    LDI R1, @SYS_WRITE
    LDI R2, 0x32
    CALL [K_SYSCALL_ENTRY]

    LDI R1, @SYS_WRITE
    LDI R2, 0x33
    CALL [K_SYSCALL_ENTRY]
    HALT
",
    );
    let mut vm = VirtualMachine::default();
    vm.load_rom(&bin);
    vm.run();
    let expected = [b'1', b'2', b'3'];
    assert_eq!(vm.devices.gpu().tty_buffer()[0..3], expected);
}

#[test]
fn test_sys_writeln() {
    let bin = assemble_kernel_test(
        r#"
start:
    LDI R1, @SYS_GPU_MODE
    LDI R2, 0x01
    CALL [K_SYSCALL_ENTRY]

    LDI R1, @SYS_WRITELN
    LDI R2:R3, HELLO_WORLD
    CALL [K_SYSCALL_ENTRY]
    HALT

HELLO_WORLD:
    .ascii "Hello, World!\0"
"#,
    );
    let mut vm = VirtualMachine::default();
    vm.load_rom(&bin);
    vm.run();
    let expected = "Hello, World!\0"
        .chars()
        .map(|c| c as u8)
        .collect::<Vec<u8>>();
    assert_eq!(vm.devices.gpu().tty_buffer()[0..14], expected);
}
