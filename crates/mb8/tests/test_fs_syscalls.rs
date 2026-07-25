use std::path::Path;

use mb8::vm::VirtualMachine;
use mb8_isa::registers::Register;

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

const FS_LIST_PROGRAM: &str = r"
start:
    LDI R1, @SYS_FS_LIST
    LDI R2, 0x01
    LDI R3, 0x50
    CALL [K_SYSCALL_ENTRY]
    HALT
";

const FS_FIND_PROGRAM: &str = r#"
start:
    LDI R1, @SYS_FS_FIND
    LDI R2:R3, FILENAME
    CALL [K_SYSCALL_ENTRY]
    HALT

FILENAME:
    .ascii "file\0"
"#;

const FS_READ_PROGRAM: &str = r#"
start:
    LDI R1, @SYS_FS_READ
    LDI R2:R3, FILENAME
    LDI R4, 0x00
    LDI R5, 0x00
    CALL [K_SYSCALL_ENTRY]
    HALT

FILENAME:
    .ascii "file\0"
"#;

#[test]
fn test_sys_fs_list() {
    let bin = assemble_kernel_test(FS_LIST_PROGRAM);
    let mut vm = VirtualMachine::default();
    let mut img = vec![0; 65536].into_boxed_slice();
    for i in 0..256 {
        img[i] = i as u8;
    }
    vm.devices.disk().set(img.try_into().unwrap());
    vm.load_rom(&bin);
    vm.run();

    for i in 0..256 {
        assert_eq!(vm.devices.read(0x0150 + i), i as u8, "{i:?}");
    }
}

#[test]
fn test_sys_fs_find() {
    let bin = assemble_kernel_test(FS_FIND_PROGRAM);
    let mut img = vec![0; 65536].into_boxed_slice();
    img[0] = 1; // status
    img[1] = 2; // start block
    img[2] = 1; // size
    img[3..8].copy_from_slice(b"file\0");

    let mut vm = VirtualMachine::default();
    vm.devices.disk().set(img.try_into().unwrap());
    vm.load_rom(&bin);
    vm.run();

    assert_eq!(vm.registers.read(Register::R1), 0);
    assert_eq!(vm.registers.read(Register::R2), 2);
    assert_eq!(vm.registers.read(Register::R3), 1);
}

#[test]
fn test_sys_fs_find_not_exist() {
    let bin = assemble_kernel_test(FS_FIND_PROGRAM);
    let mut img = vec![0; 65536].into_boxed_slice();
    img[0] = 1; // status
    img[1] = 2; // start block
    img[2] = 1; // size
    img[3..8].copy_from_slice(b"ffff\0");

    let mut vm = VirtualMachine::default();
    vm.devices.disk().set(img.try_into().unwrap());
    vm.load_rom(&bin);
    vm.run();

    assert_eq!(vm.registers.read(Register::R1), 1);
}

#[test]
fn test_sys_fs_read() {
    let bin = assemble_kernel_test(FS_READ_PROGRAM);
    let mut img = vec![0; 65536].into_boxed_slice();
    img[0] = 1; // status
    img[1] = 2; // start block
    img[2] = 2; // size
    img[3..8].copy_from_slice(b"file\0");
    img[256 * 2..256 * 4].copy_from_slice(&[1; 256 * 2]);

    let mut vm = VirtualMachine::default();
    vm.devices.disk().set(img.try_into().unwrap());
    vm.load_rom(&bin);
    vm.run();

    assert_eq!(vm.registers.read(Register::R1), 0);

    for i in 0..256 * 2 {
        assert_eq!(vm.devices.read(i), 1, "{i:?}");
    }
}

#[test]
#[ignore = "TODO"]
fn test_sys_fs_write() {
    todo!()
}

#[test]
#[ignore = "TODO"]
fn test_sys_fs_delete() {
    todo!()
}
