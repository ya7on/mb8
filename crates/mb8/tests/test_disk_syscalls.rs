use mb8::vm::VirtualMachine;

#[test]
fn test_sys_disk_set_block() {
    let bin = include_bytes!("../../../kernel/tests/test_sys_disk_set_block.bin");
    let mut vm = VirtualMachine::default();
    vm.load_rom(bin);
    vm.run();
    assert_eq!(vm.devices.read(0xF200), 0x01);
}

#[test]
fn test_sys_disk_read_block() {
    let bin = include_bytes!("../../../kernel/tests/test_sys_disk_read_block.bin");
    let mut img = vec![0; 65536].into_boxed_slice();
    for i in 0..256 {
        img[i + 256] = i as u8;
    }

    let mut vm = VirtualMachine::default();
    vm.devices.disk().set(img.try_into().unwrap());
    vm.load_rom(bin);
    vm.run();

    for i in 0..256 {
        assert_eq!(vm.devices.read(0xF202 + i), i as u8);
    }
}

#[test]
fn test_sys_disk_write_block() {
    let bin = include_bytes!("../../../kernel/tests/test_sys_disk_write_block.bin");
    let mut vm = VirtualMachine::default();
    vm.load_rom(bin);
    vm.run();

    assert_eq!(vm.devices.disk().dump()[255], 0);
    assert_eq!(vm.devices.disk().dump()[256], 228);
    assert_eq!(vm.devices.disk().dump()[257], 0);
}
