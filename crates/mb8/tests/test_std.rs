use std::path::Path;

use mb8::vm::VirtualMachine;
use mb8_isa::registers::Register;

fn assemble_rom(body: &str) -> Vec<u8> {
    let source = format!(".origin 0xE000\n\n{body}\n\n.addr 0xF000\n");
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

const MEMCPY_PROGRAM: &str = r"
start:
    LDI A, 0x00
    LDI R1, 0xFF
    LDI R2, 0x00
    LDI R3, 0x00
    LDI R4, 0x01
    LDI R5, 0x50
    MEMCPY [R4:R5], [R2:R3], R1
    HALT
";

const STRCMP_PROGRAM: &str = r"
start:
    LDI A, 0x00
    LDI R1, 0x00
    LDI R2, 0x00
    LDI R3, 0x00
    LDI R4, 0x00
    LDI R5, 0x14
    STRCMP A, R1, R2, R3, R4, R5
    HALT
";

#[test]
fn test_std_memcpy() {
    let bin = assemble_rom(MEMCPY_PROGRAM);
    let mut vm = VirtualMachine::default();

    for i in 0..256 {
        vm.devices.write(i, i as u8);
    }

    vm.load_rom(&bin);
    vm.run();

    for i in 0..256 {
        assert_eq!(vm.devices.read(0x150 + i), i as u8, "{i:?}");
    }
}

#[test]
fn test_std_strcmp_eq() {
    let bin = assemble_rom(STRCMP_PROGRAM);
    let mut vm = VirtualMachine::default();

    for i in 0..10 {
        vm.devices.write(i, 228);
        vm.devices.write(i + 0x14, 228);
    }

    vm.load_rom(&bin);
    vm.run();

    assert_eq!(vm.registers.read(Register::R0), 0);
}

#[test]
fn test_std_strcmp_neq() {
    let bin = assemble_rom(STRCMP_PROGRAM);
    let mut vm = VirtualMachine::default();

    for i in 0..10 {
        vm.devices.write(i, 228);
        vm.devices.write(i + 0x14, 228);
    }
    vm.devices.write(1, 255);

    vm.load_rom(&bin);
    vm.run();

    assert_eq!(vm.registers.read(Register::R0), 1);
}

#[test]
fn test_std_strcmp_neq_len() {
    let bin = assemble_rom(STRCMP_PROGRAM);
    let mut vm = VirtualMachine::default();

    for i in 0..10 {
        vm.devices.write(i, 228);
    }
    for i in 0..3 {
        vm.devices.write(i + 0x14, 228);
    }

    vm.load_rom(&bin);
    vm.run();

    assert_eq!(vm.registers.read(Register::R0), 1);
}
