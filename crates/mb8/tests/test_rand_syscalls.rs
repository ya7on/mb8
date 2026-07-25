use std::path::Path;

use mb8::{dev::Device, vm::VirtualMachine};

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

const RAND_PROGRAM: &str = r"
start:
    LDI R3, 0x02
    LDI R4, 0x00
    LDI R5, 0x10

rand_loop:
    LDI R1, @SYS_RAND
    CALL [K_SYSCALL_ENTRY]
    ST [R3:R4], R1

    LDI R6, 0x01
    ADD R4, R6
    LDI R6, 0x00
    CMP R4, R6
    JNZR [_skip_inc_high]
    LDI R6, 0x01
    ADD R3, R6

_skip_inc_high:
    LDI R6, 0x01
    SUB R5, R6
    LDI R6, 0x00
    CMP R5, R6
    JNZR [rand_loop]
    HALT
";

#[test]
fn test_sys_rand_deterministic() {
    let bin = assemble_kernel_test(RAND_PROGRAM);

    let mut vm1 = VirtualMachine::default();
    vm1.devices.rand().seed(234);
    vm1.load_rom(&bin);
    vm1.run();

    let mut out1 = [0u8; 16];
    for i in 0..16 {
        out1[i] = vm1.devices.read(0x0200 + i as u16);
    }

    let mut vm2 = VirtualMachine::default();
    vm2.devices.rand().seed(234);
    vm2.load_rom(&bin);
    vm2.run();

    let mut out2 = [0u8; 16];
    for i in 0..16 {
        out2[i] = vm2.devices.read(0x0200 + i as u16);
    }

    assert_eq!(out1, out2);

    let rng_value = vm1.devices.rand().read(0);
    assert_ne!(out1[0], rng_value);
}
