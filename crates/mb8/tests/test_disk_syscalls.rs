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
fn test_sys_disk_set_block() {
    let bin = assemble_kernel_test(
        r"
start:
    LDI R1, @SYS_DISK_SET_BLOCK
    LDI R2, 0x01
    CALL [K_SYSCALL_ENTRY]
    HALT
",
    );
    let mut vm = VirtualMachine::default();
    vm.load_rom(&bin);
    vm.run();
    assert_eq!(vm.devices.read(0xF200), 0x01);
}

#[test]
fn test_sys_disk_read_block() {
    let bin = assemble_kernel_test(
        r"
start:
    LDI R1, @SYS_DISK_SET_BLOCK
    LDI R2, 0x01
    CALL [K_SYSCALL_ENTRY]

    LDI R1, @SYS_DISK_READ_BLOCK
    CALL [K_SYSCALL_ENTRY]
    HALT
",
    );
    let mut img = vec![0; 65536].into_boxed_slice();
    for i in 0..256 {
        img[i + 256] = i as u8;
    }

    let mut vm = VirtualMachine::default();
    vm.devices.disk().set(img.try_into().unwrap());
    vm.load_rom(&bin);
    vm.run();

    for i in 0..256 {
        assert_eq!(vm.devices.read(0xF202 + i), i as u8);
    }
}

#[test]
fn test_sys_disk_write_block() {
    let bin = assemble_kernel_test(
        r"
start:
    LDI R1, @SYS_DISK_SET_BLOCK
    LDI R2, 0x01
    CALL [K_SYSCALL_ENTRY]

    LDI R1, 0xE4
    LDI R2, 0xF2
    LDI R3, 0x02
    ST [R2:R3], R1

    LDI R1, @SYS_DISK_WRITE_BLOCK
    CALL [K_SYSCALL_ENTRY]
    HALT
",
    );
    let mut vm = VirtualMachine::default();
    vm.load_rom(&bin);
    vm.run();

    assert_eq!(vm.devices.disk().dump()[255], 0);
    assert_eq!(vm.devices.disk().dump()[256], 228);
    assert_eq!(vm.devices.disk().dump()[257], 0);
}
