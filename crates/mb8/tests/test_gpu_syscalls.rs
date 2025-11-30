use mb8::vm::VirtualMachine;

#[test]
fn test_sys_gpu_mode() {
    let bin = include_bytes!("../../../kernel/tests/test_sys_gpu_mode.bin");
    let mut vm = VirtualMachine::default();
    vm.load_rom(bin);
    vm.run();
    assert_eq!(vm.devices.read(0xF000), 0x01);
}

#[test]
fn test_sys_write() {
    let bin = include_bytes!("../../../kernel/tests/test_sys_write.bin");
    let mut vm = VirtualMachine::default();
    vm.load_rom(bin);
    vm.run();
    let expected = [b'1', b'2', b'3'];
    assert_eq!(vm.devices.gpu().tty_buffer()[0..3], expected);
}

#[test]
fn test_sys_writeln() {
    let bin = include_bytes!("../../../kernel/tests/test_sys_writeln.bin");
    let mut vm = VirtualMachine::default();
    vm.load_rom(bin);
    vm.run();
    let expected = "Hello, World!\0"
        .chars()
        .map(|c| c as u8)
        .collect::<Vec<u8>>();
    assert_eq!(vm.devices.gpu().tty_buffer()[0..14], expected);
}
