use mb8::vm::VirtualMachine;
use mb8_isa::registers::Register;

#[test]
fn test_std_memcpy() {
    let bin = include_bytes!("../../../kernel/tests/test_std_memcpy.bin");
    let mut vm = VirtualMachine::default();

    for i in 0..256 {
        vm.devices.write(i, i as u8);
    }

    vm.load_rom(bin);
    vm.run();

    for i in 0..256 {
        assert_eq!(vm.devices.read(0x150 + i), i as u8, "{i:?}");
    }
}

#[test]
fn test_std_strcmp_eq() {
    let bin = include_bytes!("../../../kernel/tests/test_std_strcmp.bin");
    let mut vm = VirtualMachine::default();

    for i in 0..10 {
        vm.devices.write(i, 228);
        vm.devices.write(i + 0x14, 228);
    }

    vm.load_rom(bin);
    vm.run();

    assert_eq!(vm.registers.read(Register::R0), 0);
}

#[test]
fn test_std_strcmp_neq() {
    let bin = include_bytes!("../../../kernel/tests/test_std_strcmp.bin");
    let mut vm = VirtualMachine::default();

    for i in 0..10 {
        vm.devices.write(i, 228);
        vm.devices.write(i + 0x14, 228);
    }
    vm.devices.write(1, 255);

    vm.load_rom(bin);
    vm.run();

    assert_eq!(vm.registers.read(Register::R0), 1);
}

#[test]
fn test_std_strcmp_neq_len() {
    let bin = include_bytes!("../../../kernel/tests/test_std_strcmp.bin");
    let mut vm = VirtualMachine::default();

    for i in 0..10 {
        vm.devices.write(i, 228);
    }
    for i in 0..3 {
        vm.devices.write(i + 0x14, 228);
    }

    vm.load_rom(bin);
    vm.run();

    assert_eq!(vm.registers.read(Register::R0), 1);
}
